//! Web socket messages

use super::error::WSError;
use crate::state::RoomState;
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
    RoomConnect(RoomConnectInfo),
    ChooseCup(CupColor),
}

impl FromStr for WSMessage {
    type Err = WSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str::<Self>(s)
            .context("Failed to deserialize message")
            .map_err(WSError::ParseError)
    }
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/bindings/")]
pub struct RoomConnectInfo {
    pub room_name: String,
    pub connection_type: ConnectionType,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/bindings/")]
pub enum ConnectionType {
    Student,
    Teacher,
}

#[derive(Clone, Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/bindings/")]
pub enum CupColor {
    Green,
    Yellow,
    Red,
}

/// Message to respond to client
#[derive(Clone, Debug, Deserialize, Serialize, Message, TS)]
#[rtype(result = "()")]
#[serde(tag = "kind", content = "payload")]
#[ts(export, export_to = "frontend/bindings/")]
pub enum ClientMessage {
    /// General acknowledge
    Ok,
    RoomInfo(RoomInfo),
    Error(String),
}

impl ClientMessage {
    pub fn internal_error() -> Self {
        Self::Error("Internal server error".to_string())
    }
}

impl From<WSError> for ClientMessage {
    fn from(e: WSError) -> Self {
        Self::Error(e.to_string())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "frontend/bindings/")]
pub struct RoomInfo {
    pub name: String,
    pub connections: usize,
    pub green: usize,
    pub yellow: usize,
    pub red: usize,
}

impl From<RoomState> for RoomInfo {
    fn from(state: RoomState) -> Self {
        let (green, yellow, red) =
            state
                .student_connections
                .values()
                .fold((0, 0, 0), |mut acc, d| {
                    match d.cup_selection {
                        Some(CupColor::Green) => {
                            acc.0 += 1;
                        }
                        Some(CupColor::Yellow) => {
                            acc.1 += 1;
                        }
                        Some(CupColor::Red) => {
                            acc.2 += 1;
                        }
                        _ => {}
                    }
                    acc
                });
        Self {
            name: state.name,
            connections: state.student_connections.len(),
            green,
            yellow,
            red,
        }
    }
}
