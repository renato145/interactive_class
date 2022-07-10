use actix_web::web;
use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS)]
#[ts(export, export_to = "frontend/bindings/")]
pub struct CupsInfo {
    rooms: usize,
}

#[tracing::instrument()]
pub async fn get_cups() -> web::Json<CupsInfo> {
    web::Json(CupsInfo { rooms: 0 })
}
