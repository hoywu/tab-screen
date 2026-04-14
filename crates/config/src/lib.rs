use protocol::{Codec, Resolution};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct RawAppConfig {
    pub server: RawServerConfig,
    pub display: RawDisplayConfig,
    pub stream: RawStreamConfig,
    pub encoder: RawEncoderConfig,
    pub network: RawNetworkConfig,
    pub usb: RawUsbConfig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedAppConfig {
    pub server: NormalizedServerConfig,
    pub display: NormalizedDisplayConfig,
    pub stream: NormalizedStreamConfig,
    pub encoder: RawEncoderConfig,
    pub network: RawNetworkConfig,
    pub usb: RawUsbConfig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectiveAppConfig {
    pub server: NormalizedServerConfig,
    pub display: EffectiveDisplayConfig,
    pub stream: EffectiveStreamConfig,
    pub encoder: RawEncoderConfig,
    pub network: RawNetworkConfig,
    pub usb: RawUsbConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct RawServerConfig {
    pub listen_host: String,
    pub listen_port: u16,
    pub log_level: String,
    pub session_timeout_secs: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedServerConfig {
    pub listen_host: String,
    pub listen_port: u16,
    pub log_level: String,
    pub session_timeout_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct RawDisplayConfig {
    pub backend: String,
    pub name_prefix: String,
    pub create_on_client_connect: bool,
    pub destroy_on_client_disconnect: bool,
    pub identity_source: String,
    pub remember_per_client_display: bool,
    pub follow_client_screen_params: bool,
    pub force_resolution: Option<String>,
    pub force_refresh_rate: u16,
    pub force_color_depth: u8,
    pub supported_resolutions: Vec<String>,
    pub supported_refresh_rates: Vec<u16>,
    pub allowed_color_depths: Vec<u8>,
    pub dpi_mode: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedDisplayConfig {
    pub backend: String,
    pub name_prefix: String,
    pub create_on_client_connect: bool,
    pub destroy_on_client_disconnect: bool,
    pub identity_source: String,
    pub remember_per_client_display: bool,
    pub follow_client_screen_params: bool,
    pub force_resolution: Option<Resolution>,
    pub force_refresh_rate: Option<u16>,
    pub force_color_depth: Option<u8>,
    pub supported_resolutions: Vec<Resolution>,
    pub supported_refresh_rates: Vec<u16>,
    pub allowed_color_depths: Vec<u8>,
    pub dpi_mode: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectiveDisplayConfig {
    pub backend: String,
    pub name_prefix: String,
    pub create_on_client_connect: bool,
    pub destroy_on_client_disconnect: bool,
    pub identity_source: String,
    pub remember_per_client_display: bool,
    pub follow_client_screen_params: bool,
    pub force_resolution: Option<Resolution>,
    pub force_refresh_rate: Option<u16>,
    pub force_color_depth: Option<u8>,
    pub supported_resolutions: Vec<Resolution>,
    pub supported_refresh_rates: Vec<u16>,
    pub allowed_color_depths: Vec<u8>,
    pub dpi_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct RawStreamConfig {
    pub preference: RawStreamPreferenceConfig,
    pub limits: RawStreamLimitsConfig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedStreamConfig {
    pub preference: NormalizedStreamPreferenceConfig,
    pub limits: NormalizedStreamLimitsConfig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectiveStreamConfig {
    pub preference: NormalizedStreamPreferenceConfig,
    pub limits: NormalizedStreamLimitsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct RawStreamPreferenceConfig {
    pub resolution: String,
    pub frame_rate: u16,
    pub codec: Codec,
    pub bitrate_kbps: u32,
    pub encoder: String,
    pub low_latency: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedStreamPreferenceConfig {
    pub resolution: Resolution,
    pub frame_rate: u16,
    pub codec: Codec,
    pub bitrate_kbps: u32,
    pub encoder: String,
    pub low_latency: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct RawStreamLimitsConfig {
    pub allow_client_override: bool,
    pub max_resolution: String,
    pub max_frame_rate: u16,
    pub allowed_codecs: Vec<Codec>,
    pub allowed_color_depths: Vec<u8>,
    pub min_bitrate_kbps: u32,
    pub max_bitrate_kbps: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedStreamLimitsConfig {
    pub allow_client_override: bool,
    pub max_resolution: Resolution,
    pub max_frame_rate: u16,
    pub allowed_codecs: Vec<Codec>,
    pub allowed_color_depths: Vec<u8>,
    pub min_bitrate_kbps: u32,
    pub max_bitrate_kbps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct RawEncoderConfig {
    pub mode: String,
    pub device: String,
    pub gop: u16,
    pub b_frames: u8,
    pub preset: String,
    pub profile: String,
    pub tune: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct RawNetworkConfig {
    pub require_auth: bool,
    pub token: String,
    pub allow_lan: bool,
    pub allow_usb: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct RawUsbConfig {
    pub adb_path: String,
    pub reverse_port: u16,
    pub auto_reverse: bool,
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ConfigError {
    #[error("invalid resolution for field '{field}': {value}")]
    InvalidResolution { field: &'static str, value: String },
    #[error("stream bitrate range is invalid")]
    InvalidBitrateRange,
    #[error("listen_port must not be zero")]
    InvalidListenPort,
}

impl RawAppConfig {
    // Phase 0 keeps normalization local to this crate so CLI and later loaders share one path.
    pub fn normalize(&self) -> Result<NormalizedAppConfig, ConfigError> {
        Ok(NormalizedAppConfig {
            server: NormalizedServerConfig {
                listen_host: self.server.listen_host.clone(),
                listen_port: self.server.listen_port,
                log_level: self.server.log_level.clone(),
                session_timeout_secs: self.server.session_timeout_secs,
            },
            display: NormalizedDisplayConfig {
                backend: self.display.backend.clone(),
                name_prefix: self.display.name_prefix.clone(),
                create_on_client_connect: self.display.create_on_client_connect,
                destroy_on_client_disconnect: self.display.destroy_on_client_disconnect,
                identity_source: self.display.identity_source.clone(),
                remember_per_client_display: self.display.remember_per_client_display,
                follow_client_screen_params: self.display.follow_client_screen_params,
                force_resolution: parse_optional_resolution(
                    "display.force_resolution",
                    self.display.force_resolution.as_deref(),
                )?,
                force_refresh_rate: non_zero_u16(self.display.force_refresh_rate),
                force_color_depth: non_zero_u8(self.display.force_color_depth),
                supported_resolutions: self
                    .display
                    .supported_resolutions
                    .iter()
                    .map(|value| parse_resolution("display.supported_resolutions", value))
                    .collect::<Result<Vec<_>, _>>()?,
                supported_refresh_rates: self.display.supported_refresh_rates.clone(),
                allowed_color_depths: self.display.allowed_color_depths.clone(),
                dpi_mode: self.display.dpi_mode.clone(),
            },
            stream: NormalizedStreamConfig {
                preference: NormalizedStreamPreferenceConfig {
                    resolution: parse_resolution(
                        "stream.preference.resolution",
                        &self.stream.preference.resolution,
                    )?,
                    frame_rate: self.stream.preference.frame_rate,
                    codec: self.stream.preference.codec,
                    bitrate_kbps: self.stream.preference.bitrate_kbps,
                    encoder: self.stream.preference.encoder.clone(),
                    low_latency: self.stream.preference.low_latency,
                },
                limits: NormalizedStreamLimitsConfig {
                    allow_client_override: self.stream.limits.allow_client_override,
                    max_resolution: parse_resolution(
                        "stream.limits.max_resolution",
                        &self.stream.limits.max_resolution,
                    )?,
                    max_frame_rate: self.stream.limits.max_frame_rate,
                    allowed_codecs: self.stream.limits.allowed_codecs.clone(),
                    allowed_color_depths: self.stream.limits.allowed_color_depths.clone(),
                    min_bitrate_kbps: self.stream.limits.min_bitrate_kbps,
                    max_bitrate_kbps: self.stream.limits.max_bitrate_kbps,
                },
            },
            encoder: self.encoder.clone(),
            network: self.network.clone(),
            usb: self.usb.clone(),
        })
    }
}

impl NormalizedAppConfig {
    // Validation produces the runtime-safe model and rejects impossible defaults early.
    pub fn validate(self) -> Result<EffectiveAppConfig, ConfigError> {
        if self.server.listen_port == 0 {
            return Err(ConfigError::InvalidListenPort);
        }

        if self.stream.limits.min_bitrate_kbps > self.stream.limits.max_bitrate_kbps {
            return Err(ConfigError::InvalidBitrateRange);
        }

        Ok(EffectiveAppConfig {
            server: self.server,
            display: EffectiveDisplayConfig {
                backend: self.display.backend,
                name_prefix: self.display.name_prefix,
                create_on_client_connect: self.display.create_on_client_connect,
                destroy_on_client_disconnect: self.display.destroy_on_client_disconnect,
                identity_source: self.display.identity_source,
                remember_per_client_display: self.display.remember_per_client_display,
                follow_client_screen_params: self.display.follow_client_screen_params,
                force_resolution: self.display.force_resolution,
                force_refresh_rate: self.display.force_refresh_rate,
                force_color_depth: self.display.force_color_depth,
                supported_resolutions: self.display.supported_resolutions,
                supported_refresh_rates: self.display.supported_refresh_rates,
                allowed_color_depths: self.display.allowed_color_depths,
                dpi_mode: self.display.dpi_mode,
            },
            stream: EffectiveStreamConfig {
                preference: self.stream.preference,
                limits: self.stream.limits,
            },
            encoder: self.encoder,
            network: self.network,
            usb: self.usb,
        })
    }
}

pub fn load_effective_config(raw: RawAppConfig) -> Result<EffectiveAppConfig, ConfigError> {
    raw.normalize()?.validate()
}

pub fn default_config_toml() -> Result<String, toml::ser::Error> {
    toml::to_string_pretty(&RawAppConfig::default())
}

fn parse_resolution(field: &'static str, value: &str) -> Result<Resolution, ConfigError> {
    value.parse().map_err(|_| ConfigError::InvalidResolution {
        field,
        value: value.to_owned(),
    })
}

fn parse_optional_resolution(
    field: &'static str,
    value: Option<&str>,
) -> Result<Option<Resolution>, ConfigError> {
    match value.map(str::trim) {
        None | Some("") => Ok(None),
        Some(value) => parse_resolution(field, value).map(Some),
    }
}

fn non_zero_u16(value: u16) -> Option<u16> {
    (value != 0).then_some(value)
}

fn non_zero_u8(value: u8) -> Option<u8> {
    (value != 0).then_some(value)
}

impl Default for RawAppConfig {
    fn default() -> Self {
        Self {
            server: RawServerConfig::default(),
            display: RawDisplayConfig::default(),
            stream: RawStreamConfig::default(),
            encoder: RawEncoderConfig::default(),
            network: RawNetworkConfig::default(),
            usb: RawUsbConfig::default(),
        }
    }
}

impl Default for RawServerConfig {
    fn default() -> Self {
        Self {
            listen_host: "0.0.0.0".to_owned(),
            listen_port: 38491,
            log_level: "info".to_owned(),
            session_timeout_secs: 15,
        }
    }
}

impl Default for RawDisplayConfig {
    fn default() -> Self {
        Self {
            backend: "auto".to_owned(),
            name_prefix: "Tab Screen".to_owned(),
            create_on_client_connect: true,
            destroy_on_client_disconnect: true,
            identity_source: "client_stable_id".to_owned(),
            remember_per_client_display: true,
            follow_client_screen_params: true,
            force_resolution: None,
            force_refresh_rate: 0,
            force_color_depth: 0,
            supported_resolutions: vec!["1920x1200".to_owned(), "2560x1600".to_owned()],
            supported_refresh_rates: vec![60],
            allowed_color_depths: vec![8],
            dpi_mode: "follow-client".to_owned(),
        }
    }
}

impl Default for RawStreamConfig {
    fn default() -> Self {
        Self {
            preference: RawStreamPreferenceConfig::default(),
            limits: RawStreamLimitsConfig::default(),
        }
    }
}

impl Default for RawStreamPreferenceConfig {
    fn default() -> Self {
        Self {
            resolution: "1920x1200".to_owned(),
            frame_rate: 60,
            codec: Codec::H264,
            bitrate_kbps: 12_000,
            encoder: "auto".to_owned(),
            low_latency: true,
        }
    }
}

impl Default for RawStreamLimitsConfig {
    fn default() -> Self {
        Self {
            allow_client_override: true,
            max_resolution: "2560x1600".to_owned(),
            max_frame_rate: 60,
            allowed_codecs: vec![Codec::H264, Codec::Hevc],
            allowed_color_depths: vec![8],
            min_bitrate_kbps: 2_000,
            max_bitrate_kbps: 30_000,
        }
    }
}

impl Default for RawEncoderConfig {
    fn default() -> Self {
        Self {
            mode: "auto".to_owned(),
            device: String::new(),
            gop: 60,
            b_frames: 0,
            preset: "low-latency".to_owned(),
            profile: "high".to_owned(),
            tune: "zerolatency".to_owned(),
        }
    }
}

impl Default for RawNetworkConfig {
    fn default() -> Self {
        Self {
            require_auth: true,
            token: "change-me".to_owned(),
            allow_lan: true,
            allow_usb: true,
        }
    }
}

impl Default for RawUsbConfig {
    fn default() -> Self {
        Self {
            adb_path: "adb".to_owned(),
            reverse_port: 38491,
            auto_reverse: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_matches_documented_phase_zero_baseline() {
        let raw = RawAppConfig::default();

        assert_eq!(raw.server.listen_port, 38491);
        assert_eq!(raw.display.name_prefix, "Tab Screen");
        assert_eq!(raw.stream.preference.codec, Codec::H264);
        assert_eq!(raw.usb.adb_path, "adb");
    }

    #[test]
    fn default_config_can_be_normalized_and_validated() {
        let effective = load_effective_config(RawAppConfig::default()).unwrap();

        assert_eq!(
            effective.stream.preference.resolution,
            Resolution::new(1920, 1200)
        );
        assert_eq!(
            effective.stream.limits.max_resolution,
            Resolution::new(2560, 1600)
        );
    }

    #[test]
    fn default_config_toml_contains_expected_sections() {
        let config = default_config_toml().unwrap();

        assert!(config.contains("[server]"));
        assert!(config.contains("[display]"));
        assert!(config.contains("[stream.preference]"));
        assert!(config.contains("[usb]"));
    }
}
