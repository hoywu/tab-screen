use anyhow::Result;
use capture::{RawFrame, RawFrameFormat};
use protocol::{Codec, Resolution};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncoderProbe {
    pub backend_name: String,
    pub available_codecs: Vec<Codec>,
    pub hardware_accelerated: bool,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncoderConfig {
    pub codec: Codec,
    pub input_format: RawFrameFormat,
    pub resolution: Resolution,
    pub frame_rate: u16,
    pub bitrate_kbps: u32,
    pub gop: u16,
    pub low_latency: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EncoderReconfigure {
    pub resolution: Option<Resolution>,
    pub frame_rate: Option<u16>,
    pub bitrate_kbps: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReconfigureOutcome {
    AppliedHot,
    RequiresRestart,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncodedFrame {
    pub codec: Codec,
    pub pts_us: u64,
    pub keyframe: bool,
    pub config: bool,
    pub bytes: Vec<u8>,
}

pub trait EncoderBackend: Send {
    fn backend_name(&self) -> &'static str;
    fn probe(&self) -> Result<EncoderProbe>;
    fn start(&mut self, config: EncoderConfig) -> Result<()>;
    fn encode(&mut self, frame: RawFrame) -> Result<EncodedFrame>;
    fn reconfigure(&mut self, update: EncoderReconfigure) -> Result<ReconfigureOutcome>;
    fn stop(&mut self) -> Result<()>;
}
