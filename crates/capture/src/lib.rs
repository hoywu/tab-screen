use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    Rgba8888,
    Bgra8888,
    Nv12,
}

impl PixelFormat {
    pub fn bytes_per_pixel(self) -> Option<u32> {
        match self {
            Self::Rgba8888 | Self::Bgra8888 => Some(4),
            Self::Nv12 => None,
        }
    }

    pub fn is_packed(self) -> bool {
        self.bytes_per_pixel().is_some()
    }

    pub fn packed_frame_size(self, width: u32, height: u32, stride: u32) -> Result<usize> {
        let bytes_per_pixel = self
            .bytes_per_pixel()
            .ok_or_else(|| anyhow!("pixel format does not use a packed single-plane layout"))?;

        if width == 0 || height == 0 {
            return Err(anyhow!("frame dimensions must be non-zero"));
        }

        let min_stride = width
            .checked_mul(bytes_per_pixel)
            .ok_or_else(|| anyhow!("minimum stride calculation overflowed"))?;

        if stride < min_stride {
            return Err(anyhow!(
                "stride {stride} is smaller than minimum packed stride {min_stride}"
            ));
        }

        let total_bytes = stride
            .checked_mul(height)
            .ok_or_else(|| anyhow!("packed frame size calculation overflowed"))?;

        usize::try_from(total_bytes)
            .map_err(|_| anyhow!("packed frame size does not fit into usize"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawFrameFormat {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub pixel_format: PixelFormat,
    pub color_depth: u8,
}

impl RawFrameFormat {
    pub fn packed_frame_size(&self) -> Result<usize> {
        self.pixel_format
            .packed_frame_size(self.width, self.height, self.stride)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed_rgba_frame_size_uses_stride_times_height() {
        let size = PixelFormat::Rgba8888
            .packed_frame_size(1920, 1080, 7680)
            .expect("rgba packed size should be valid");

        assert_eq!(size, 8_294_400);
    }

    #[test]
    fn packed_bgra_frame_size_allows_padding_in_stride() {
        let size = PixelFormat::Bgra8888
            .packed_frame_size(3, 2, 16)
            .expect("bgra packed size with padding should be valid");

        assert_eq!(size, 32);
    }

    #[test]
    fn packed_frame_size_rejects_zero_dimensions() {
        let error = PixelFormat::Rgba8888
            .packed_frame_size(0, 1080, 7680)
            .expect_err("zero width must fail");

        assert!(error.to_string().contains("non-zero"));
    }

    #[test]
    fn packed_frame_size_rejects_stride_smaller_than_minimum() {
        let error = PixelFormat::Bgra8888
            .packed_frame_size(10, 10, 39)
            .expect_err("undersized stride must fail");

        assert!(
            error
                .to_string()
                .contains("smaller than minimum packed stride 40")
        );
    }

    #[test]
    fn packed_frame_size_rejects_non_packed_format() {
        let error = PixelFormat::Nv12
            .packed_frame_size(1920, 1080, 1920)
            .expect_err("nv12 is not a packed single-plane format");

        assert!(
            error
                .to_string()
                .contains("does not use a packed single-plane layout")
        );
    }

    #[test]
    fn raw_frame_format_delegates_packed_frame_size() {
        let format = RawFrameFormat {
            width: 1280,
            height: 720,
            stride: 5120,
            pixel_format: PixelFormat::Rgba8888,
            color_depth: 8,
        };

        let size = format
            .packed_frame_size()
            .expect("raw frame format should compute packed size");

        assert_eq!(size, 3_686_400);
    }
}
