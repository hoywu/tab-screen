use protocol::{ErrorCode, ProtocolError, PROTOCOL_VERSION};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionLifecycleState {
    Idle,
    ClientConnected,
    Handshaking,
    DisplayProvisioning,
    Negotiating,
    Streaming,
    Renegotiating,
    Terminating,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionStatus {
    pub state: SessionLifecycleState,
    pub session_id: Option<Uuid>,
    pub client_id: Option<Uuid>,
}

impl Default for SessionStatus {
    fn default() -> Self {
        Self {
            state: SessionLifecycleState::Idle,
            session_id: None,
            client_id: None,
        }
    }
}

#[derive(Debug, Default)]
pub struct SessionManager {
    status: SessionStatus,
}

impl SessionManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status(&self) -> &SessionStatus {
        &self.status
    }

    // Phase 0 only needs a serial state holder so CLI and later transport code share one boundary.
    pub fn begin_handshake(&mut self, client_id: Uuid) {
        self.status = SessionStatus {
            state: SessionLifecycleState::Handshaking,
            session_id: Some(Uuid::new_v4()),
            client_id: Some(client_id),
        };
    }

    pub fn transition_to(&mut self, state: SessionLifecycleState) {
        self.status.state = state;
    }

    pub fn clear(&mut self) {
        self.status = SessionStatus::default();
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("{message}")]
pub struct ServerError {
    pub code: ErrorCode,
    pub message: String,
    pub recoverable: bool,
}

impl ServerError {
    pub fn new(code: ErrorCode, message: impl Into<String>, recoverable: bool) -> Self {
        Self {
            code,
            message: message.into(),
            recoverable,
        }
    }

    pub fn to_protocol_error(&self) -> ProtocolError {
        ProtocolError {
            code: self.code,
            message: self.message.clone(),
            recoverable: self.recoverable,
        }
    }
}

pub fn validate_protocol_version(version: u32) -> Result<(), ServerError> {
    if version == PROTOCOL_VERSION {
        Ok(())
    } else {
        Err(ServerError::new(
            ErrorCode::ProtocolVersionMismatch,
            format!("unsupported protocol version {version}, expected {PROTOCOL_VERSION}"),
            false,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_version_validation_rejects_mismatch() {
        let error = validate_protocol_version(PROTOCOL_VERSION + 1).unwrap_err();

        assert_eq!(error.code, ErrorCode::ProtocolVersionMismatch);
    }
}
