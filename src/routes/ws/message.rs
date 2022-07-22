//! Web socket messages

use super::error::WSError;
use crate::state::{QuestionState, RoomState};
use actix::Message;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};
use ts_rs::TS;
use uuid::Uuid;

/// Message from client
#[derive(Deserialize, Message, TS)]
#[rtype(result = "()")]
#[serde(tag = "task", content = "payload")]
#[ts(export, export_to = "frontend/bindings/")]
pub enum WSMessage {
    RoomConnect(RoomConnectInfo),
    ChooseCup(CupColor),
    CreateQuestion(Question),
    PublishQuestion(PublishQuestion),
    DeleteQuestion(QuestionId),
    ModifyQuestion(QuestionModification),
    AnswerQuestion(QuestionAnswer),
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

#[derive(Clone, Copy, Debug, Deserialize, TS)]
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

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/bindings/")]
pub struct Question {
    pub title: String,
    pub options: Vec<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/bindings/")]
pub struct PublishQuestion {
    pub id: QuestionId,
    /// Seconds for the question to be available to students
    pub secs: usize,
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
    QuestionsInfo(Vec<QuestionInfo>),
    QuestionPublication(QuestionPublication),
    QuestionDelete(QuestionId),
    Error(String),
}

impl ClientMessage {
    pub fn internal_error() -> Self {
        Self::Error("Internal server error".to_string())
    }

    pub fn from_questions_map(questions: HashMap<Uuid, QuestionState>) -> Self {
        let all_info = questions
            .into_iter()
            .map(|(id, question_state)| {
                let answers = question_state.summary();
                QuestionInfo {
                    id: QuestionId(id),
                    title: question_state.title,
                    options: question_state.options,
                    answers,
                }
            })
            .collect();
        Self::QuestionsInfo(all_info)
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

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "frontend/bindings/")]
pub struct QuestionInfo {
    pub id: QuestionId,
    pub title: String,
    pub options: Vec<String>,
    pub answers: Vec<usize>,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "frontend/bindings/")]
pub struct QuestionPublication {
    pub id: QuestionId,
    pub title: String,
    pub options: Vec<String>,
    /// Seconds for the question to be available to students
    pub secs: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS, PartialEq, Eq)]
#[ts(export, export_to = "frontend/bindings/")]
pub struct QuestionId(#[ts(type = "string")] pub Uuid);

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "frontend/bindings/")]
pub struct QuestionModification {
    #[ts(type = "string")]
    pub id: Uuid,
    pub title: Option<String>,
    pub options: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "frontend/bindings/")]
pub struct QuestionAnswer {
    #[ts(type = "string")]
    pub id: Uuid,
    pub answer: usize,
}
