use std::collections::HashSet;

use crate::{error_chain_fmt, state::AppState, utils::e400};
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(thiserror::Error)]
pub enum CupsError {
    #[error("Room {0:?} already exists.")]
    RoomAlreadyExists(String),
    #[error("Something went wrong.")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for CupsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq, TS)]
#[ts(export, export_to = "frontend/bindings/")]
pub struct CupsInfo {
    pub rooms: HashSet<String>,
}

#[tracing::instrument(skip_all)]
pub async fn get_cups(state: web::Data<AppState>) -> web::Json<CupsInfo> {
    let cups_info = CupsInfo {
        rooms: state.rooms.lock().unwrap().clone(),
    };
    web::Json(cups_info)
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "frontend/bindings/")]
pub struct CreateRoom {
    new_room: String,
}

#[tracing::instrument(skip_all)]
pub async fn create_room(
    form: web::Json<CreateRoom>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let room_name = form.into_inner().new_room;
    let mut rooms = state.rooms.lock().unwrap();
    if rooms.insert(room_name.clone()) {
        Ok(HttpResponse::Ok().finish())
    } else {
        Err(e400(CupsError::RoomAlreadyExists(room_name)))
    }
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq, TS)]
#[ts(export, export_to = "frontend/bindings/")]
pub struct RoomInfo {
    pub name: String,
}

impl RoomInfo {
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

/// Get room information, if it doesn't exists it creates a new room
#[tracing::instrument()]
#[get("/{room}")]
pub async fn get_cups_room(path: web::Path<String>) -> web::Json<RoomInfo> {
    web::Json(RoomInfo::new(path.into_inner()))
}
