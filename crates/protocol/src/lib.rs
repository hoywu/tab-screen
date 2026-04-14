use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;
use uuid::Uuid;

pub const PROTOCOL_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl Resolution {
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl fmt::Display for Resolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("invalid resolution '{value}', expected <width>x<height>")]
pub struct ResolutionParseError {
    value: String,
}

impl FromStr for Resolution {
    type Err = ResolutionParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (width, height) = value.split_once('x').ok_or_else(|| ResolutionParseError {
            value: value.to_owned(),
        })?;

        let width = width.parse::<u32>().map_err(|_| ResolutionParseError {
            value: value.to_owned(),
        })?;
        let height = height.parse::<u32>().map_err(|_| ResolutionParseError {
            value: value.to_owned(),
        })?;

        Ok(Self { width, height })
    }
}

impl Serialize for Resolution {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Resolution {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NetworkMode {
    Lan,
    Usb,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Codec {
    H264,
    Hevc,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Orientation {
    Landscape,
    Portrait,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SessionState {
    Connecting,
    Negotiating,
    Streaming,
    Renegotiating,
    Error,
    Idle,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    AuthFailed,
    ProtocolVersionMismatch,
    InvalidConfig,
    MissingDeviceScreenParams,
    DisplayBackendUnavailable,
    DisplayCreationFailed,
    EncoderUnavailable,
    DecoderUnavailable,
    ParameterOutOfRange,
    SessionBusy,
    NetworkDisconnected,
    SessionTimeout,
    InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceScreenParams {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u16,
    pub color_depth: u8,
    pub orientation: Orientation,
    pub dpi: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DecodeCapabilities {
    pub codecs: Vec<Codec>,
    pub max_width: u32,
    pub max_height: u32,
    pub max_frame_rate: u16,
    pub hardware_decode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StreamPreference {
    pub resolution: Resolution,
    pub frame_rate: u16,
    pub codec: Codec,
    pub bitrate_kbps: u32,
    pub low_latency: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StreamLimits {
    pub max_resolution: Resolution,
    pub max_frame_rate: u16,
    pub allowed_codecs: Vec<Codec>,
    pub min_bitrate_kbps: u32,
    pub max_bitrate_kbps: u32,
    pub allow_client_override: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StreamOverrideRequest {
    pub resolution: Option<Resolution>,
    pub frame_rate: Option<u16>,
    pub codec: Option<Codec>,
    pub bitrate_kbps: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EffectiveDisplayParams {
    pub resolution: Resolution,
    pub refresh_rate: u16,
    pub color_depth: u8,
    pub dpi: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EffectiveStreamParams {
    pub resolution: Resolution,
    pub frame_rate: u16,
    pub codec: Codec,
    pub bitrate_kbps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DowngradeReason {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtocolError {
    pub code: ErrorCode,
    pub message: String,
    pub recoverable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ControlMessage {
    ClientHello {
        protocol_version: u32,
        client_stable_id: Uuid,
        device_model: String,
        network_mode: NetworkMode,
        device_screen: DeviceScreenParams,
        decode_caps: DecodeCapabilities,
    },
    ServerHello {
        protocol_version: u32,
        server_version: String,
        display_backend: String,
        available_codecs: Vec<Codec>,
        display_name: String,
        stream_preference: StreamPreference,
        stream_limits: StreamLimits,
    },
    StartSessionRequest {
        follow_server_preference: bool,
        stream_override: Option<StreamOverrideRequest>,
    },
    StartSessionResponse {
        accepted: bool,
        session_id: Option<Uuid>,
        display_params: Option<EffectiveDisplayParams>,
        effective_stream: Option<EffectiveStreamParams>,
        downgrade_reasons: Vec<DowngradeReason>,
        error: Option<ProtocolError>,
    },
    UpdateStreamRequest {
        session_id: Uuid,
        stream_override: StreamOverrideRequest,
    },
    UpdateStreamResponse {
        accepted: bool,
        session_id: Uuid,
        effective_stream: Option<EffectiveStreamParams>,
        downgrade_reasons: Vec<DowngradeReason>,
        error: Option<ProtocolError>,
    },
    Heartbeat {
        session_id: Option<Uuid>,
        timestamp_ms: u64,
    },
    Error {
        error: ProtocolError,
    },
    SessionEnded {
        session_id: Option<Uuid>,
        reason: Option<String>,
    },
    SessionState {
        state: SessionState,
        session_id: Option<Uuid>,
        detail: Option<String>,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MediaPacketType {
    Video,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct MediaPacketFlags {
    pub keyframe: bool,
    pub config: bool,
    pub end_of_stream: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MediaPacketHeader {
    pub packet_type: MediaPacketType,
    pub codec: Codec,
    pub flags: MediaPacketFlags,
    pub pts_us: u64,
    pub payload_len: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn resolution_round_trips_as_string() {
        let resolution = Resolution::new(1920, 1200);

        let serialized = serde_json::to_string(&resolution).unwrap();
        let decoded: Resolution = serde_json::from_str(&serialized).unwrap();

        assert_eq!(serialized, "\"1920x1200\"");
        assert_eq!(decoded, resolution);
    }

    #[test]
    fn client_hello_serializes_with_internal_tag() {
        let message = ControlMessage::ClientHello {
            protocol_version: PROTOCOL_VERSION,
            client_stable_id: Uuid::nil(),
            device_model: "SM-X700".to_owned(),
            network_mode: NetworkMode::Lan,
            device_screen: DeviceScreenParams {
                width: 2560,
                height: 1600,
                refresh_rate: 60,
                color_depth: 8,
                orientation: Orientation::Landscape,
                dpi: 280,
            },
            decode_caps: DecodeCapabilities {
                codecs: vec![Codec::H264, Codec::Hevc],
                max_width: 2560,
                max_height: 1600,
                max_frame_rate: 60,
                hardware_decode: true,
            },
        };

        let value = serde_json::to_value(message).unwrap();

        assert_eq!(value["type"], json!("client_hello"));
        assert_eq!(value["device_model"], json!("SM-X700"));
        assert_eq!(value["decode_caps"]["codecs"], json!(["h264", "hevc"]));
    }
}
