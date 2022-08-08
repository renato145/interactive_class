use crate::{
    error_chain_fmt,
    state::{AppState, RoomState},
    utils::e400,
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use ts_rs::TS;

#[derive(thiserror::Error)]
pub enum CupsError {
    #[error("Room {0:?} already exists.")]
    RoomAlreadyExists(String),
    #[error("Room {0:?} doesn't exists.")]
    NoExistingRoom(String),
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
pub async fn get_cups_info(state: web::Data<AppState>) -> web::Json<CupsInfo> {
    let rooms = state.rooms.lock().unwrap().keys().cloned().collect();
    let cups_info = CupsInfo { rooms };
    web::Json(cups_info)
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/bindings/")]
pub struct CreateRoom {
    new_room: String,
}

#[tracing::instrument(skip(state))]
pub async fn create_room(
    form: web::Json<CreateRoom>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let room_name = form.into_inner().new_room;
    let mut rooms = state.rooms.lock().unwrap();
    if rooms
        .insert(room_name.clone(), RoomState::new(room_name.clone()))
        .is_none()
    {
        Ok(HttpResponse::Ok().finish())
    } else {
        Err(e400(CupsError::RoomAlreadyExists(room_name)))
    }
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/bindings/")]
pub struct DeleteRoom {
    room: String,
}

/// Deletes a room and returns the new room information
#[tracing::instrument(skip(state))]
pub async fn delete_room(
    form: web::Json<DeleteRoom>,
    state: web::Data<AppState>,
) -> Result<web::Json<CupsInfo>, actix_web::Error> {
    let room_name = form.into_inner().room;
    let mut rooms = state.rooms.lock().unwrap();
    if rooms.remove(&room_name).is_some() {
        let rooms = rooms.keys().cloned().collect();
        let cups_info = CupsInfo { rooms };
        Ok(web::Json(cups_info))
    } else {
        Err(e400(CupsError::NoExistingRoom(room_name)))
    }
}
