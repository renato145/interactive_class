use crate::{error_chain_fmt, state::StateError};
use uuid::Uuid;

#[derive(thiserror::Error)]
pub enum WSError {
    #[error("Client already connected..")]
    AlreadyConnected,
    #[error("No connected to any room.")]
    NoRoom,
    #[error("Invalid room: {0:?}.")]
    InvalidRoom(String),
    #[error("Failed to parse websocket message.")]
    ParseError(#[source] anyhow::Error),
    #[error("{0}")]
    InvalidClientId(#[source] StateError),
    #[error("Invalid question id: {0:?}.")]
    InvalidQuestionId(Uuid),
    #[error("{0}")]
    InvalidAnswer(#[source] StateError),
}

impl std::fmt::Debug for WSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl From<StateError> for WSError {
    fn from(e: StateError) -> Self {
        match e {
            StateError::InvalidId => Self::InvalidClientId(e),
            StateError::InvalidAnswer(_) => Self::InvalidAnswer(e),
        }
    }
}
