use anyhow::{Context, Result, anyhow, bail};
use capture::{CaptureSource, PixelFormat, RawFrame, RawFrameFormat};
use std::ffi::c_void;
use std::fmt;

use std::os::raw::{c_char, c_int, c_uint};
use std::path::Path;
use std::ptr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualDisplaySpec {
    pub logical_name: String,
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u16,
    pub color_depth: u8,
    pub dpi: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayBackendProbe {
    pub backend_name: String,
    pub supports_stable_naming: bool,
    pub supports_create_destroy: bool,
    pub notes: Vec<String>,
}

pub trait DisplayBackend: Send + Sync {
    fn backend_name(&self) -> &'static str;
    fn probe(&self) -> Result<DisplayBackendProbe>;
    fn supports_stable_naming(&self) -> bool;
    fn create_output(&self, spec: VirtualDisplaySpec) -> Result<Box<dyn DisplayHandle>>;
}

pub trait DisplayHandle: Send {
    fn logical_name(&self) -> &str;
    fn backend_id(&self) -> &str;
    fn capture_source(&self) -> Result<Box<dyn CaptureSource>>;
    fn destroy(self: Box<Self>) -> Result<()>;
}

/// Phase 0 placeholder backend kept for fallback and tests.
#[derive(Debug, Default)]
pub struct NoopDisplayBackend;

impl DisplayBackend for NoopDisplayBackend {
    fn backend_name(&self) -> &'static str {
        "noop"
    }

    fn probe(&self) -> Result<DisplayBackendProbe> {
        Ok(DisplayBackendProbe {
            backend_name: self.backend_name().to_owned(),
            supports_stable_naming: false,
            supports_create_destroy: false,
            notes: vec!["Phase 0 placeholder backend".to_owned()],
        })
    }

    fn supports_stable_naming(&self) -> bool {
        false
    }

    fn create_output(&self, spec: VirtualDisplaySpec) -> Result<Box<dyn DisplayHandle>> {
        Ok(Box::new(NoopDisplayHandle {
            logical_name: spec.logical_name,
        }))
    }
}

#[derive(Debug)]
pub struct NoopDisplayHandle {
    logical_name: String,
}

impl DisplayHandle for NoopDisplayHandle {
    fn logical_name(&self) -> &str {
        &self.logical_name
    }

    fn backend_id(&self) -> &str {
        "noop:placeholder"
    }

    fn capture_source(&self) -> Result<Box<dyn CaptureSource>> {
        bail!("noop backend does not provide a capture source")
    }

    fn destroy(self: Box<Self>) -> Result<()> {
        Ok(())
    }
}

/// Primary Phase 1 backend based on the kernel-backed EVDI path.
#[derive(Debug, Default)]
pub struct EvdiDisplayBackend;

impl EvdiDisplayBackend {
    fn lib_version() -> Result<EvdiLibVersion> {
        let mut version = EvdiLibVersion::default();
        unsafe {
            // We query the linked libevdi version up front so doctor/probe can report it.
            evdi_get_lib_version(&mut version);
        }

        if version.version_major <= 0 {
            bail!("libevdi returned an invalid version");
        }

        Ok(version)
    }

    fn module_loaded() -> bool {
        Path::new("/sys/module/evdi").exists()
    }

    fn dri_directory_exists() -> bool {
        Path::new("/dev/dri").exists()
    }

    fn sysfs_virtual_drm_exists() -> bool {
        Path::new("/sys/devices/virtual/drm").exists()
    }

    fn privilege_note() -> &'static str {
        "dynamic evdi node creation/opening commonly requires elevated privileges; prefer a system-level privileged service model"
    }
}

impl DisplayBackend for EvdiDisplayBackend {
    fn backend_name(&self) -> &'static str {
        "evdi"
    }

    fn probe(&self) -> Result<DisplayBackendProbe> {
        let version = Self::lib_version().context("failed to query libevdi version")?;
        let mut notes = vec![
            format!("linked libevdi version: {}", version),
            Self::privilege_note().to_owned(),
            "uses evdi_open_attached_to_fixed instead of deprecated evdi_open_attached_to".to_owned(),
            "stable logical identity is expected to come from mapping + generated EDID monitor identity, not fixed card numbering".to_owned(),
            "Arch Linux prerequisite: install evdi-dkms".to_owned(),
            "Arch Linux prerequisite: install linux-headers".to_owned(),
            "Arch Linux prerequisite: create /etc/modules-load.d/evdi.conf with 'evdi' to auto-load the kernel module".to_owned(),
        ];

        if Self::module_loaded() {
            notes.push("kernel module: /sys/module/evdi present".to_owned());
        } else {
            notes.push("kernel module: /sys/module/evdi missing".to_owned());
        }

        if Self::dri_directory_exists() {
            notes.push("/dev/dri present".to_owned());
        } else {
            notes.push("/dev/dri missing".to_owned());
        }

        if Self::sysfs_virtual_drm_exists() {
            notes.push("/sys/devices/virtual/drm present".to_owned());
        } else {
            notes.push("/sys/devices/virtual/drm missing".to_owned());
        }

        Ok(DisplayBackendProbe {
            backend_name: self.backend_name().to_owned(),
            supports_stable_naming: true,
            supports_create_destroy: Self::module_loaded(),
            notes,
        })
    }

    fn supports_stable_naming(&self) -> bool {
        true
    }

    fn create_output(&self, spec: VirtualDisplaySpec) -> Result<Box<dyn DisplayHandle>> {
        spec.validate()?;

        let mut logical_name = spec.logical_name.clone();
        logical_name.truncate(13);

        let edid = generate_evdi_edid(&spec)
            .context("failed to generate EDID for evdi virtual display")?;

        let handle = unsafe {
            // The installed evdi header explicitly recommends the fixed helper.
            evdi_open_attached_to_fixed(ptr::null(), 0)
        };

        if handle.is_null() {
            bail!(
                "evdi_open_attached_to_fixed returned an invalid handle; ensure libevdi is available, the evdi module is loaded, and the process has sufficient privileges"
            );
        }

        let device = EvdiDevice::new(handle, spec.clone(), edid)?;
        let backend_id = format!("evdi:{}", device.debug_id());

        device.connect().context("failed to connect evdi display")?;

        Ok(Box::new(EvdiDisplayHandle {
            logical_name: spec.logical_name,
            backend_id,
            device: Arc::new(device),
        }))
    }
}

#[derive(Debug)]
pub struct EvdiDisplayHandle {
    logical_name: String,
    backend_id: String,
    device: Arc<EvdiDevice>,
}

impl DisplayHandle for EvdiDisplayHandle {
    fn logical_name(&self) -> &str {
        &self.logical_name
    }

    fn backend_id(&self) -> &str {
        &self.backend_id
    }

    fn capture_source(&self) -> Result<Box<dyn CaptureSource>> {
        Ok(Box::new(EvdiCaptureSource::new(Arc::clone(&self.device))))
    }

    fn destroy(self: Box<Self>) -> Result<()> {
        if Arc::strong_count(&self.device) > 1 {
            bail!("cannot destroy evdi display while a capture source is still active");
        }

        self.device
            .disconnect_and_close()
            .context("failed to destroy evdi display")?;

        Ok(())
    }
}

#[derive(Debug)]
struct EvdiDevice {
    handle: Mutex<EvdiHandle>,
    spec: VirtualDisplaySpec,
    edid: Vec<u8>,
    connected: Mutex<bool>,
}

impl EvdiDevice {
    fn new(
        handle: *mut EvdiDeviceContext,
        spec: VirtualDisplaySpec,
        edid: Vec<u8>,
    ) -> Result<Self> {
        Ok(Self {
            handle: Mutex::new(EvdiHandle::Open(EvdiHandlePtr::new(handle)?)),
            spec,
            edid,
            connected: Mutex::new(false),
        })
    }

    fn debug_id(&self) -> String {
        short_hash_hex(self.spec.logical_name.as_bytes())
    }

    fn handle_ptr(&self) -> Result<*mut EvdiDeviceContext> {
        match *self.handle.lock().expect("evdi handle mutex poisoned") {
            EvdiHandle::Open(ptr) => Ok(ptr.as_ptr()),
            EvdiHandle::Closed => bail!("evdi handle is already closed"),
        }
    }

    fn connect(&self) -> Result<()> {
        let handle = self.handle_ptr()?;
        unsafe {
            // The EDID announces the monitor identity and preferred mode to DRM.
            evdi_connect(
                handle,
                self.edid.as_ptr(),
                u32::try_from(self.edid.len()).context("EDID length overflowed u32")?,
                self.spec
                    .width
                    .checked_mul(self.spec.height)
                    .ok_or_else(|| {
                        anyhow!("pixel area overflowed while connecting evdi display")
                    })?,
            );
        }

        *self
            .connected
            .lock()
            .expect("evdi connected mutex poisoned") = true;
        Ok(())
    }

    fn disconnect_and_close(&self) -> Result<()> {
        let mut handle_guard = self.handle.lock().expect("evdi handle mutex poisoned");
        let handle = match *handle_guard {
            EvdiHandle::Open(ptr) => ptr.as_ptr(),
            EvdiHandle::Closed => return Ok(()),
        };

        if *self
            .connected
            .lock()
            .expect("evdi connected mutex poisoned")
        {
            unsafe {
                // We disconnect first so the virtual monitor disappears cleanly from DRM.
                evdi_disconnect(handle);
            }
            *self
                .connected
                .lock()
                .expect("evdi connected mutex poisoned") = false;
        }

        unsafe {
            // Closing releases the userspace association with the evdi node.
            evdi_close(handle);
        }

        *handle_guard = EvdiHandle::Closed;
        Ok(())
    }
}

impl Drop for EvdiDevice {
    fn drop(&mut self) {
        let _ = self.disconnect_and_close();
    }
}

#[derive(Debug, Clone, Copy)]
enum EvdiHandle {
    Open(EvdiHandlePtr),
    Closed,
}

#[derive(Debug, Clone, Copy)]
struct EvdiHandlePtr(std::ptr::NonNull<EvdiDeviceContext>);

impl EvdiHandlePtr {
    fn new(ptr: *mut EvdiDeviceContext) -> Result<Self> {
        let non_null =
            std::ptr::NonNull::new(ptr).ok_or_else(|| anyhow!("evdi handle pointer is null"))?;
        Ok(Self(non_null))
    }

    fn as_ptr(self) -> *mut EvdiDeviceContext {
        self.0.as_ptr()
    }
}

// The underlying libevdi handle is only accessed behind synchronization and is treated
// as an opaque kernel/userspace handle in this Phase 1 implementation.
unsafe impl Send for EvdiHandlePtr {}

#[derive(Debug)]
struct EvdiCaptureSource {
    device: Arc<EvdiDevice>,
    last_format: Option<RawFrameFormat>,
}

impl EvdiCaptureSource {
    fn new(device: Arc<EvdiDevice>) -> Self {
        Self {
            device,
            last_format: None,
        }
    }

    fn wait_for_mode(&self, timeout: Duration) -> Result<EvdiMode> {
        let handle = self.device.handle_ptr()?;
        let deadline = Instant::now() + timeout;
        let mut event_state = CaptureEventState::default();

        while Instant::now() < deadline {
            self.pump_events(handle, &mut event_state)
                .context("failed while waiting for evdi mode event")?;

            if let Some(mode) = event_state.mode.take() {
                if mode.width > 0 && mode.height > 0 && mode.bits_per_pixel >= 24 {
                    return Ok(mode);
                }
            }

            std::thread::sleep(Duration::from_millis(25));
        }

        bail!(
            "timed out waiting for evdi mode_changed event; ensure the compositor activated the virtual display"
        )
    }

    fn pump_events(
        &self,
        handle: *mut EvdiDeviceContext,
        state: &mut CaptureEventState,
    ) -> Result<()> {
        let mut event_context = EvdiEventContext {
            dpms_handler: None,
            mode_changed_handler: Some(mode_changed_handler),
            update_ready_handler: Some(update_ready_handler),
            crtc_state_handler: None,
            cursor_set_handler: None,
            cursor_move_handler: None,
            ddcci_data_handler: None,
            user_data: state as *mut CaptureEventState as *mut c_void,
        };

        unsafe {
            // EVDI uses an event-dispatch model; this drains any queued notifications.
            evdi_handle_events(handle, &mut event_context);
        }

        Ok(())
    }

    fn wait_for_update(
        &self,
        handle: *mut EvdiDeviceContext,
        state: &mut CaptureEventState,
        buffer_id: i32,
        timeout: Duration,
    ) -> Result<()> {
        let immediate = unsafe { evdi_request_update(handle, buffer_id as c_int) };
        if immediate {
            state.update_ready = true;
            return Ok(());
        }

        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            self.pump_events(handle, state)
                .context("failed while waiting for evdi update_ready event")?;

            if state.update_ready {
                return Ok(());
            }

            std::thread::sleep(Duration::from_millis(25));
        }

        bail!(
            "timed out waiting for evdi update_ready event; ensure visible content is rendered on the virtual display"
        )
    }
}

impl CaptureSource for EvdiCaptureSource {
    fn frame_format(&self) -> RawFrameFormat {
        self.last_format.unwrap_or(RawFrameFormat {
            width: self.device.spec.width,
            height: self.device.spec.height,
            stride: self.device.spec.width.saturating_mul(4),
            pixel_format: PixelFormat::Bgra8888,
            color_depth: self.device.spec.color_depth.max(8),
        })
    }

    fn next_frame(&mut self) -> Result<RawFrame> {
        let handle = self.device.handle_ptr()?;
        let mode = self.wait_for_mode(Duration::from_secs(5))?;
        let format =
            raw_frame_format_from_mode(mode).context("unsupported evdi mode for capture")?;
        let frame_bytes = format
            .packed_frame_size()
            .context("failed to calculate evdi capture buffer size")?;

        let mut backing = vec![0_u8; frame_bytes];
        let mut rects = vec![EvdiRect::default(); 16];
        let mut buffer = EvdiBuffer {
            id: 1,
            buffer: backing.as_mut_ptr() as *mut c_void,
            width: i32_from_u32(format.width)?,
            height: i32_from_u32(format.height)?,
            stride: i32_from_u32(format.stride)?,
            rects: rects.as_mut_ptr(),
            rect_count: i32_from_usize(rects.len())?,
        };

        unsafe {
            // We register one simple userspace buffer for Phase 1 validation capture.
            evdi_register_buffer(handle, buffer);
        }

        let result = (|| {
            let mut state = CaptureEventState {
                mode: Some(mode),
                update_ready: false,
            };

            self.wait_for_update(handle, &mut state, buffer.id, Duration::from_secs(5))?;

            unsafe {
                // Once update_ready fires, grab_pixels copies the latest framebuffer into our buffer.
                evdi_grab_pixels(handle, buffer.rects, &mut buffer.rect_count);
            }

            if buffer.rect_count <= 0 {
                bail!("evdi returned no dirty rectangles for the captured frame");
            }

            if backing.is_empty() {
                bail!("evdi capture buffer is empty");
            }

            self.last_format = Some(format);

            Ok(RawFrame::new(format, 0, backing.clone()))
        })();

        unsafe {
            // Buffer registration lifetime is scoped to the single capture attempt.
            evdi_unregister_buffer(handle, buffer.id);
        }

        result
    }
}

#[derive(Debug, Default)]
struct CaptureEventState {
    mode: Option<EvdiMode>,
    update_ready: bool,
}

unsafe extern "C" fn mode_changed_handler(mode: EvdiMode, user_data: *mut c_void) {
    let state_ptr = user_data as *mut CaptureEventState;
    if let Some(state) = unsafe { state_ptr.as_mut() } {
        state.mode = Some(mode);
    }
}

unsafe extern "C" fn update_ready_handler(_buffer_to_be_updated: c_int, user_data: *mut c_void) {
    let state_ptr = user_data as *mut CaptureEventState;
    if let Some(state) = unsafe { state_ptr.as_mut() } {
        state.update_ready = true;
    }
}

fn raw_frame_format_from_mode(mode: EvdiMode) -> Result<RawFrameFormat> {
    let width = u32::try_from(mode.width).context("evdi mode width was negative")?;
    let height = u32::try_from(mode.height).context("evdi mode height was negative")?;
    let bits_per_pixel =
        u8::try_from(mode.bits_per_pixel).context("evdi bits_per_pixel overflowed u8")?;

    if width == 0 || height == 0 {
        bail!("evdi mode dimensions must be non-zero");
    }

    if bits_per_pixel != 32 {
        bail!("only 32-bit packed evdi capture is supported in Phase 1; got {bits_per_pixel} bpp");
    }

    let stride = width
        .checked_mul(4)
        .ok_or_else(|| anyhow!("evdi stride calculation overflowed"))?;

    Ok(RawFrameFormat {
        width,
        height,
        stride,
        pixel_format: PixelFormat::Bgra8888,
        color_depth: 8,
    })
}

impl VirtualDisplaySpec {
    fn validate(&self) -> Result<()> {
        if self.logical_name.trim().is_empty() {
            bail!("logical display name must not be empty");
        }
        if self.width == 0 || self.height == 0 {
            bail!("virtual display dimensions must be non-zero");
        }
        if self.refresh_rate == 0 {
            bail!("virtual display refresh rate must be non-zero");
        }
        if self.color_depth < 8 {
            bail!("virtual display color depth must be at least 8-bit");
        }
        Ok(())
    }
}

fn generate_evdi_edid(spec: &VirtualDisplaySpec) -> Result<Vec<u8>> {
    spec.validate()?;

    let mut edid = [0_u8; 128];

    // Header.
    edid[0..8].copy_from_slice(&[0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00]);

    // Manufacturer + product + serial. We generate deterministic values from logical identity.
    let manufacturer = pack_eisa_id("TSC");
    edid[8] = (manufacturer >> 8) as u8;
    edid[9] = (manufacturer & 0xff) as u8;

    let product_code = stable_u16_from_name(&spec.logical_name);
    edid[10..12].copy_from_slice(&product_code.to_le_bytes());

    let serial = stable_u32_from_name(&spec.logical_name);
    edid[12..16].copy_from_slice(&serial.to_le_bytes());

    edid[16] = 1; // manufacture week
    edid[17] = 35; // model year offset => 2025
    edid[18] = 0x01;
    edid[19] = 0x04;

    // Basic display parameters.
    edid[20] = 0x80; // digital input
    edid[21] = size_cm(spec.width, spec.dpi)?;
    edid[22] = size_cm(spec.height, spec.dpi)?;
    edid[23] = 0x78; // gamma 2.2 approximation
    edid[24] = 0x0a; // RGB color, preferred timing mode

    // Chromaticity left intentionally generic for Phase 1 validation.
    edid[25..35].copy_from_slice(&[0u8; 10]);

    // Established / standard timings intentionally left blank; preferred detailed timing is enough.
    edid[35..54].copy_from_slice(&[0x01; 19]);

    // Preferred detailed timing descriptor.
    let detailed_timing = build_detailed_timing(spec)?;
    edid[54..72].copy_from_slice(&detailed_timing);

    // Monitor name descriptor.
    let name_descriptor = build_text_descriptor(0xfc, &format_monitor_name(&spec.logical_name));
    edid[72..90].copy_from_slice(&name_descriptor);

    // Serial descriptor.
    let serial_descriptor = build_text_descriptor(
        0xff,
        &format!("TS{}", short_hash_hex(spec.logical_name.as_bytes())),
    );
    edid[90..108].copy_from_slice(&serial_descriptor);

    // Range limits descriptor.
    let range_descriptor = build_range_limits_descriptor(spec.refresh_rate);
    edid[108..126].copy_from_slice(&range_descriptor);

    edid[126] = 0; // extension count
    edid[127] = checksum_byte(&edid);

    Ok(edid.to_vec())
}

fn build_detailed_timing(spec: &VirtualDisplaySpec) -> Result<[u8; 18]> {
    let h_active = spec.width;
    let v_active = spec.height;

    // Use a simple CVT-RB-like timing envelope suitable for Phase 1 validation.
    let h_blanking = 160_u32;
    let v_blanking = 35_u32;
    let h_sync_offset = 48_u32;
    let h_sync_width = 32_u32;
    let v_sync_offset = 3_u32;
    let v_sync_width = 5_u32;

    let h_total = h_active
        .checked_add(h_blanking)
        .ok_or_else(|| anyhow!("horizontal total overflowed"))?;
    let v_total = v_active
        .checked_add(v_blanking)
        .ok_or_else(|| anyhow!("vertical total overflowed"))?;

    let pixel_clock_hz = h_total
        .checked_mul(v_total)
        .and_then(|v| v.checked_mul(spec.refresh_rate as u32))
        .ok_or_else(|| anyhow!("pixel clock overflowed"))?;
    let pixel_clock_khz = pixel_clock_hz / 1_000;
    let pixel_clock_10khz = u16::try_from(pixel_clock_khz / 10)
        .context("pixel clock does not fit in EDID detailed timing")?;

    let h_blank = u16::try_from(h_blanking).context("horizontal blanking overflowed u16")?;
    let v_blank = u16::try_from(v_blanking).context("vertical blanking overflowed u16")?;
    let h_active_u16 = u16::try_from(h_active).context("horizontal active overflowed u16")?;
    let v_active_u16 = u16::try_from(v_active).context("vertical active overflowed u16")?;
    let h_sync_offset_u16 =
        u16::try_from(h_sync_offset).context("horizontal sync offset overflowed u16")?;
    let h_sync_width_u16 =
        u16::try_from(h_sync_width).context("horizontal sync width overflowed u16")?;
    let v_sync_offset_u8 =
        u8::try_from(v_sync_offset).context("vertical sync offset overflowed u8")?;
    let v_sync_width_u8 =
        u8::try_from(v_sync_width).context("vertical sync width overflowed u8")?;

    let horizontal_mm = size_mm(spec.width, spec.dpi)?;
    let vertical_mm = size_mm(spec.height, spec.dpi)?;

    let mut dtd = [0_u8; 18];
    dtd[0..2].copy_from_slice(&pixel_clock_10khz.to_le_bytes());
    dtd[2] = (h_active_u16 & 0xff) as u8;
    dtd[3] = (h_blank & 0xff) as u8;
    dtd[4] = (((h_active_u16 >> 8) & 0x0f) as u8) << 4 | (((h_blank >> 8) & 0x0f) as u8);
    dtd[5] = (v_active_u16 & 0xff) as u8;
    dtd[6] = (v_blank & 0xff) as u8;
    dtd[7] = (((v_active_u16 >> 8) & 0x0f) as u8) << 4 | (((v_blank >> 8) & 0x0f) as u8);
    dtd[8] = (h_sync_offset_u16 & 0xff) as u8;
    dtd[9] = (h_sync_width_u16 & 0xff) as u8;
    dtd[10] = ((v_sync_offset_u8 & 0x0f) << 4) | (v_sync_width_u8 & 0x0f);
    dtd[11] = ((((h_sync_offset_u16 >> 8) & 0x03) as u8) << 6)
        | ((((h_sync_width_u16 >> 8) & 0x03) as u8) << 4)
        | (((v_sync_offset_u8 >> 4) & 0x03) << 2)
        | ((v_sync_width_u8 >> 4) & 0x03);
    dtd[12] = (horizontal_mm & 0xff) as u8;
    dtd[13] = (vertical_mm & 0xff) as u8;
    dtd[14] = (((horizontal_mm >> 8) & 0x0f) as u8) << 4 | (((vertical_mm >> 8) & 0x0f) as u8);
    dtd[15] = 0;
    dtd[16] = 0;
    dtd[17] = 0x1a; // no stereo, digital separate sync

    Ok(dtd)
}

fn build_text_descriptor(tag: u8, text: &str) -> [u8; 18] {
    let mut descriptor = [0_u8; 18];
    descriptor[0] = 0x00;
    descriptor[1] = 0x00;
    descriptor[2] = 0x00;
    descriptor[3] = tag;
    descriptor[4] = 0x00;

    let mut bytes = [b' '; 13];
    let trimmed = text.as_bytes();
    let count = trimmed.len().min(13);
    bytes[..count].copy_from_slice(&trimmed[..count]);

    if count < 13 {
        bytes[count] = b'\n';
    }

    descriptor[5..18].copy_from_slice(&bytes);
    descriptor
}

fn build_range_limits_descriptor(refresh_rate: u16) -> [u8; 18] {
    let mut descriptor = [0_u8; 18];
    descriptor[0] = 0x00;
    descriptor[1] = 0x00;
    descriptor[2] = 0x00;
    descriptor[3] = 0xfd;
    descriptor[4] = 0x00;
    descriptor[5] = refresh_rate.saturating_sub(5).clamp(24, 240) as u8;
    descriptor[6] = refresh_rate.clamp(24, 240) as u8;
    descriptor[7] = 30;
    descriptor[8] = 160;
    descriptor[9] = 0x00;
    descriptor
}

fn pack_eisa_id(id: &str) -> u16 {
    let bytes = id.as_bytes();
    let a = bytes.get(0).copied().unwrap_or(b'A') - b'@';
    let b = bytes.get(1).copied().unwrap_or(b'A') - b'@';
    let c = bytes.get(2).copied().unwrap_or(b'A') - b'@';

    ((a as u16) << 10) | ((b as u16) << 5) | (c as u16)
}

fn size_mm(pixels: u32, dpi: u16) -> Result<u16> {
    if dpi == 0 {
        bail!("dpi must be non-zero for EDID physical size calculation");
    }

    let mm = ((pixels as f64) * 25.4 / (dpi as f64)).round();
    u16::try_from(mm as u32).context("physical size overflowed u16")
}

fn size_cm(pixels: u32, dpi: u16) -> Result<u8> {
    let mm = size_mm(pixels, dpi)?;
    let cm = (mm as f64 / 10.0).round();
    u8::try_from(cm as u32).context("physical size overflowed u8 centimeters")
}

fn checksum_byte(edid: &[u8; 128]) -> u8 {
    let sum: u32 = edid[..127].iter().map(|value| u32::from(*value)).sum();
    ((256 - (sum % 256)) % 256) as u8
}

fn stable_u32_from_name(name: &str) -> u32 {
    fnv1a32(name.as_bytes())
}

fn stable_u16_from_name(name: &str) -> u16 {
    (fnv1a32(name.as_bytes()) & 0xffff) as u16
}

fn short_hash_hex(bytes: &[u8]) -> String {
    format!("{:08X}", fnv1a32(bytes))
}

fn fnv1a32(bytes: &[u8]) -> u32 {
    let mut hash = 0x811C9DC5_u32;
    for byte in bytes {
        hash ^= u32::from(*byte);
        hash = hash.wrapping_mul(0x01000193);
    }
    hash
}

fn format_monitor_name(name: &str) -> String {
    let trimmed = name.trim();
    let mut result = if trimmed.is_empty() {
        "Tab Screen".to_owned()
    } else {
        trimmed.to_owned()
    };
    result.truncate(13);
    result
}

fn i32_from_u32(value: u32) -> Result<i32> {
    i32::try_from(value).context("value does not fit in i32")
}

fn i32_from_usize(value: usize) -> Result<i32> {
    i32::try_from(value).context("value does not fit in i32")
}

#[repr(C)]
struct EvdiDeviceContext {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
struct EvdiLibVersion {
    version_major: c_int,
    version_minor: c_int,
    version_patchlevel: c_int,
}

impl fmt::Display for EvdiLibVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            self.version_major, self.version_minor, self.version_patchlevel
        )
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
struct EvdiRect {
    x1: c_int,
    y1: c_int,
    x2: c_int,
    y2: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
struct EvdiMode {
    width: c_int,
    height: c_int,
    refresh_rate: c_int,
    bits_per_pixel: c_int,
    pixel_format: c_uint,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct EvdiBuffer {
    id: c_int,
    buffer: *mut c_void,
    width: c_int,
    height: c_int,
    stride: c_int,
    rects: *mut EvdiRect,
    rect_count: c_int,
}

#[repr(C)]
struct EvdiEventContext {
    dpms_handler: Option<unsafe extern "C" fn(c_int, *mut c_void)>,
    mode_changed_handler: Option<unsafe extern "C" fn(EvdiMode, *mut c_void)>,
    update_ready_handler: Option<unsafe extern "C" fn(c_int, *mut c_void)>,
    crtc_state_handler: Option<unsafe extern "C" fn(c_int, *mut c_void)>,
    cursor_set_handler: Option<unsafe extern "C" fn(EvdiCursorSet, *mut c_void)>,
    cursor_move_handler: Option<unsafe extern "C" fn(EvdiCursorMove, *mut c_void)>,
    ddcci_data_handler: Option<unsafe extern "C" fn(EvdiDdcciData, *mut c_void)>,
    user_data: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct EvdiCursorSet {
    hot_x: i32,
    hot_y: i32,
    width: u32,
    height: u32,
    enabled: u8,
    buffer_length: u32,
    buffer: *mut u32,
    pixel_format: u32,
    stride: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct EvdiCursorMove {
    x: i32,
    y: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct EvdiDdcciData {
    address: u16,
    flags: u16,
    buffer_length: u32,
    buffer: *mut u8,
}

#[link(name = "evdi")]
unsafe extern "C" {
    fn evdi_open_attached_to_fixed(
        sysfs_parent_device: *const c_char,
        length: usize,
    ) -> *mut EvdiDeviceContext;
    fn evdi_close(handle: *mut EvdiDeviceContext);
    fn evdi_connect(
        handle: *mut EvdiDeviceContext,
        edid: *const u8,
        edid_length: u32,
        sku_area_limit: u32,
    );
    fn evdi_disconnect(handle: *mut EvdiDeviceContext);
    fn evdi_register_buffer(handle: *mut EvdiDeviceContext, buffer: EvdiBuffer);
    fn evdi_unregister_buffer(handle: *mut EvdiDeviceContext, buffer_id: c_int);
    fn evdi_request_update(handle: *mut EvdiDeviceContext, buffer_id: c_int) -> bool;
    fn evdi_grab_pixels(
        handle: *mut EvdiDeviceContext,
        rects: *mut EvdiRect,
        num_rects: *mut c_int,
    );
    fn evdi_handle_events(handle: *mut EvdiDeviceContext, evtctx: *mut EvdiEventContext);
    fn evdi_get_lib_version(version: *mut EvdiLibVersion);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_spec() -> VirtualDisplaySpec {
        VirtualDisplaySpec {
            logical_name: "Tab Screen Test".to_owned(),
            width: 1920,
            height: 1200,
            refresh_rate: 60,
            color_depth: 8,
            dpi: 160,
        }
    }

    #[test]
    fn edid_checksum_is_valid() {
        let edid = generate_evdi_edid(&sample_spec()).expect("edid generation should succeed");
        assert_eq!(edid.len(), 128);

        let sum: u32 = edid.iter().map(|value| u32::from(*value)).sum();
        assert_eq!(sum % 256, 0);
    }

    #[test]
    fn edid_is_stable_for_same_logical_name() {
        let first = generate_evdi_edid(&sample_spec()).expect("first edid should succeed");
        let second = generate_evdi_edid(&sample_spec()).expect("second edid should succeed");
        assert_eq!(first, second);
    }

    #[test]
    fn edid_changes_when_logical_name_changes() {
        let first = generate_evdi_edid(&sample_spec()).expect("first edid should succeed");

        let mut changed = sample_spec();
        changed.logical_name = "Another Tablet".to_owned();

        let second = generate_evdi_edid(&changed).expect("second edid should succeed");
        assert_ne!(first[10..16], second[10..16]);
    }

    #[test]
    fn format_monitor_name_truncates_to_edid_limit() {
        assert_eq!(format_monitor_name("0123456789ABCDE"), "0123456789ABC");
    }

    #[test]
    fn raw_frame_format_from_evdi_mode_accepts_32bpp() {
        let format = raw_frame_format_from_mode(EvdiMode {
            width: 2560,
            height: 1600,
            refresh_rate: 60,
            bits_per_pixel: 32,
            pixel_format: 0,
        })
        .expect("32-bit evdi mode should be supported");

        assert_eq!(format.width, 2560);
        assert_eq!(format.height, 1600);
        assert_eq!(format.stride, 10_240);
        assert_eq!(format.pixel_format, PixelFormat::Bgra8888);
    }

    #[test]
    fn raw_frame_format_from_evdi_mode_rejects_non_32bpp() {
        let error = raw_frame_format_from_mode(EvdiMode {
            width: 1920,
            height: 1080,
            refresh_rate: 60,
            bits_per_pixel: 24,
            pixel_format: 0,
        })
        .expect_err("non-32-bit evdi mode must be rejected");

        assert!(error.to_string().contains("32-bit"));
    }

    #[test]
    fn virtual_display_spec_validation_rejects_zero_values() {
        let mut spec = sample_spec();
        spec.width = 0;

        let error = spec.validate().expect_err("zero width must be invalid");
        assert!(error.to_string().contains("non-zero"));
    }

    #[test]
    fn short_hash_hex_is_stable() {
        assert_eq!(short_hash_hex(b"tablet-a"), short_hash_hex(b"tablet-a"));
    }
}
