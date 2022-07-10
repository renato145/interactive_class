use actix_web::{get, web};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq, TS)]
#[ts(export, export_to = "frontend/bindings/")]
pub struct CupsInfo {
    pub rooms: usize,
}

#[tracing::instrument()]
pub async fn get_cups() -> web::Json<CupsInfo> {
    web::Json(CupsInfo { rooms: 0 })
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
