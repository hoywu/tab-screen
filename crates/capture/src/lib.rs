use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    Rgba8888,
    Bgra8888,
    Nv12,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawFrameFormat {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub pixel_format: PixelFormat,
    pub color_depth: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawFrame {
    pub format: RawFrameFormat,
    pub pts_us: u64,
    pub bytes: Vec<u8>,
}

impl RawFrame {
    pub fn new(format: RawFrameFormat, pts_us: u64, bytes: Vec<u8>) -> Self {
        Self {
            format,
            pts_us,
            bytes,
        }
    }
}

pub trait CaptureSource: Send {
    fn frame_format(&self) -> RawFrameFormat;
    fn next_frame(&mut self) -> Result<RawFrame>;
}
