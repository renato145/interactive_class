//! Web socket messages

use super::error::WSError;
use actix::Message;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use ts_rs::TS;

#[derive(Deserialize, TS)]
#[ts(export, export_to = "frontend/bindings/")]
pub enum WSTask {
    RoomConnect,
}

/// Message from client
#[derive(Deserialize, Message, TS)]
#[rtype(result = "()")]
#[ts(export, export_to = "frontend/bindings/")]
pub struct WSMessage {
    pub task: WSTask,
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
#[derive(Serialize, Message, TS)]
#[rtype(result = "()")]
#[ts(export, export_to = "frontend/bindings/")]
pub struct ClientMessage {
    pub success: bool,
    pub payload: Option<String>,
}

impl ClientMessage {
    pub fn success() -> Self {
        Self {
            success: true,
            payload: None,
        }
    }
}

impl From<WSError> for ClientMessage {
    fn from(e: WSError) -> Self {
        Self {
            success: false,
            payload: Some(e.to_string()),
        }
    }
}
