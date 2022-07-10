//! Inspired by cups.fast.ai
mod error;
mod message;
mod session;
// mod server;

use self::session::WSSession;
use crate::configuration::WSSettings;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

#[tracing::instrument(name = "Starting web socket", skip_all)]
pub async fn ws(
    req: HttpRequest,
    stream: web::Payload,
    settings: web::Data<WSSettings>,
) -> Result<HttpResponse, Error> {
    ws::start(WSSession::new(settings.as_ref().clone()), &req, stream)
}
