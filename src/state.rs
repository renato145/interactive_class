use crate::routes::message::{ClientMessage, Question};
use crate::{error_chain_fmt, routes::message::CupColor};
use actix::Recipient;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::{collections::HashMap, sync::Mutex};
use uuid::Uuid;

#[derive(thiserror::Error)]
pub enum StateError {
    #[error("Invalid client id.")]
    InvalidId,
    #[error("Invalid answer: {0}.")]
    InvalidAnswer(usize),
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
    /// QuestionId -> QuestionState
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QuestionState {
    pub title: String,
    pub options: Vec<String>,
    /// StudentId -> answer idx
    pub answers: HashMap<Uuid, usize>,
}

impl QuestionState {
    pub fn new(title: String, options: Vec<String>) -> Self {
        Self {
            title,
            options,
            answers: HashMap::new(),
        }
    }

    pub fn answer(&mut self, student_id: Uuid, answer: usize) -> Result<(), StateError> {
        if answer > self.options.len() {
            Err(StateError::InvalidAnswer(answer))
        } else {
            self.answers.insert(student_id, answer);
            Ok(())
        }
    }

    pub fn modify(&mut self, title: Option<String>, options: Option<Vec<String>>) {
        if let Some(title) = title {
            self.title = title;
        }
        if let Some(options) = options {
            // Check which answers to keep
            let prev2new = self
                .answers
                .values()
                .collect::<HashSet<_>>()
                .into_iter()
                .filter_map(|&previous_idx| {
                    let current_answer = &self.options[previous_idx];
                    options
                        .iter()
                        .position(|o| o == current_answer)
                        .map(|new_idx| (previous_idx, new_idx))
                })
                .collect::<HashMap<_, _>>();
            // Modify current answers
            self.answers = self
                .answers
                .iter()
                .filter_map(|(&id, &answer)| prev2new.get(&answer).map(|&new_idx| (id, new_idx)))
                .collect();
            // Set new options
            self.options = options;
        }
    }

    /// Get #answers for each option
    pub fn summary(&self) -> Vec<usize> {
        let counts = self.answers.values().counts();
        (0..self.options.len())
            .map(|i| counts.get(&i).cloned().unwrap_or(0))
            .collect()
    }
}

impl From<Question> for QuestionState {
    fn from(question: Question) -> Self {
        Self::new(question.title, question.options)
    }
}
