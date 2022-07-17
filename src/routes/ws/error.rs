use crate::error_chain_fmt;

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
}

impl std::fmt::Debug for WSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
