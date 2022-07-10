//! Web socket messages

use super::error::WSError;
use actix::Message;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Deserialize)]
pub enum WSTask {
    RoomConnect,
}

/// Message from client
#[derive(Deserialize, Message)]
#[rtype(result = "()")]
pub struct WSMessage {
    pub task: WSTask,
    pub payload: Option<serde_json::Value>,
}

impl FromStr for WSMessage {
    type Err = WSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str::<Self>(s)
            .context("Failed to deserialize message")
            .map_err(WSError::ParseError)
    }
}

/// Message to respond to client
#[derive(Serialize, Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub success: bool,
    pub payload: serde_json::Value,
}

impl From<WSError> for ClientMessage {
    fn from(e: WSError) -> Self {
        Self {
            success: false,
            payload: e.to_string().into(),
        }
    }
}
