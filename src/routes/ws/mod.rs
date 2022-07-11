//! Inspired by cups.fast.ai
mod error;
pub mod message;
mod session;

use self::session::WSSession;
use crate::{configuration::WSSettings, state::AppState};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

#[tracing::instrument(name = "Starting web socket", skip_all)]
pub async fn ws(
    req: HttpRequest,
    stream: web::Payload,
    settings: web::Data<WSSettings>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WSSession::new(state, settings.as_ref().clone()),
        &req,
        stream,
    )
}
