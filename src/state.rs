use crate::routes::message::{ClientMessage, Question};
use crate::{error_chain_fmt, routes::message::CupColor};
use actix::Recipient;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Mutex};
use ts_rs::TS;
use uuid::Uuid;

#[derive(thiserror::Error)]
pub enum StateError {
    #[error("Invalid client id.")]
    InvalidId,
}

impl std::fmt::Debug for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[derive(Default, Debug)]
pub struct AppState {
    pub rooms: Mutex<HashMap<String, RoomState>>,
}

#[derive(Debug, Clone)]
pub struct RoomState {
    pub name: String,
    pub student_connections: HashMap<Uuid, StudentInfo>,
    pub teacher_connections: HashMap<Uuid, Recipient<ClientMessage>>,
    pub questions: HashMap<Uuid, QuestionState>,
}

impl RoomState {
    pub fn new(name: String) -> Self {
        Self {
            name,
            student_connections: HashMap::new(),
            teacher_connections: HashMap::new(),
            questions: HashMap::new(),
        }
    }

    pub fn choose_cup(&mut self, id: &Uuid, color: CupColor) -> Result<(), StateError> {
        match self.student_connections.get_mut(id) {
            Some(data) => {
                data.cup_selection = Some(color);
                Ok(())
            }
            None => Err(StateError::InvalidId),
        }
    }

    pub fn add_question(&mut self, question: Question) {
        self.questions.insert(Uuid::new_v4(), question.into());
    }
}

#[derive(Debug, Clone)]
pub struct StudentInfo {
    pub connection: Recipient<ClientMessage>,
    pub cup_selection: Option<CupColor>,
}

impl StudentInfo {
    pub fn new(connection: Recipient<ClientMessage>) -> Self {
        Self {
            connection,
            cup_selection: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export, export_to = "frontend/bindings/")]
pub struct QuestionState {
    pub title: String,
    pub options: Vec<String>,
    pub answers: Vec<usize>,
}

impl QuestionState {
    pub fn new(title: String, options: Vec<String>) -> Self {
        let answers = vec![0; options.len()];
        Self {
            title,
            options,
            answers,
        }
    }
}

impl From<Question> for QuestionState {
    fn from(question: Question) -> Self {
        Self::new(question.title, question.options)
    }
}
