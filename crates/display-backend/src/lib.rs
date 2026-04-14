use anyhow::{bail, Result};
use capture::CaptureSource;

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

// Phase 0 exposes a no-op backend so higher layers can compile before Phase 1 selects a real backend.
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
