use protocol::PROTOCOL_VERSION;

pub const SESSION_PATH: &str = "/session";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeartbeatConfig {
    pub interval_secs: u64,
    pub timeout_secs: u64,
}

impl Default for HeartbeatConfig {
    fn default() -> Self {
        Self {
            interval_secs: 5,
            timeout_secs: 15,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportServerConfig {
    pub listen_host: String,
    pub listen_port: u16,
    pub session_path: &'static str,
    pub protocol_version: u32,
    pub heartbeat: HeartbeatConfig,
}

impl Default for TransportServerConfig {
    fn default() -> Self {
        Self {
            listen_host: "0.0.0.0".to_owned(),
            listen_port: 38491,
            session_path: SESSION_PATH,
            protocol_version: PROTOCOL_VERSION,
            heartbeat: HeartbeatConfig::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportServer {
    pub config: TransportServerConfig,
}

impl TransportServer {
    pub fn new(config: TransportServerConfig) -> Self {
        Self { config }
    }

    pub fn describe(&self) -> String {
        format!(
            "ws://{}:{}{} (protocol v{})",
            self.config.listen_host,
            self.config.listen_port,
            self.config.session_path,
            self.config.protocol_version
        )
    }
}
