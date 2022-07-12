//! Web socket messages

use super::error::WSError;
use actix::Message;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use ts_rs::TS;

/// Message from client
#[derive(Deserialize, Message, TS)]
#[rtype(result = "()")]
#[serde(tag = "task", content = "payload")]
#[ts(export, export_to = "frontend/bindings/")]
pub enum WSMessage {
    RoomConnect(String),
}

impl FromStr for WSMessage {
    type Err = WSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        tracing::debug!(s);
        serde_json::from_str::<Self>(s)
            .context("Failed to deserialize message")
            .map_err(WSError::ParseError)
    }
}

/// Message to respond to client
#[derive(Debug, Deserialize, Serialize, Message, TS)]
#[rtype(result = "()")]
#[serde(tag = "kind", content = "payload")]
#[ts(export, export_to = "frontend/bindings/")]
pub enum ClientMessage {
    RoomInfo(RoomInfo),
    Error(String),
}

impl ClientMessage {
    pub fn internal_error() -> Self {
        Self::Error("Internal server error".to_string())
    }
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "frontend/bindings/")]
pub struct RoomInfo {
    pub name: String,
    pub connections: usize,
}

impl From<WSError> for ClientMessage {
    fn from(e: WSError) -> Self {
        Self::Error(e.to_string())
    }
}
