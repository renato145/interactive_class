use actix_web::web;
use serde::Serialize;

#[derive(Serialize)]
pub struct CupsInfo {
    rooms: usize,
}

#[tracing::instrument()]
pub async fn get_cups() -> web::Json<CupsInfo> {
    web::Json(CupsInfo { rooms: 0 })
}
