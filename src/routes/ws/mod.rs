//! Inspired by cups.fast.ai
mod error;
mod message;

use crate::configuration::WSSettings;
use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use message::WSMessage;
use std::{str::FromStr, time::Instant};

#[tracing::instrument(name = "Starting web socket", skip_all)]
pub async fn ws(
    req: HttpRequest,
    stream: web::Payload,
    settings: web::Data<WSSettings>,
) -> Result<HttpResponse, Error> {
    ws::start(WSSession::new(settings.as_ref().clone()), &req, stream)
}

struct WSSession {
    hb: Instant,
    settings: WSSettings,
}

impl WSSession {
    fn new(settings: WSSettings) -> Self {
        Self {
            hb: Instant::now(),
            settings,
        }
    }

    /// Sends ping to client every x seconds.
    /// Also checks heartbeats from client.
    #[tracing::instrument(name = "heartbeat", skip_all)]
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(self.settings.heartbeat_interval, |act, ctx| {
            // Check client heartbeats
            if Instant::now().duration_since(act.hb) > act.settings.client_timeout {
                // heartbeat timed out
                tracing::info!("Websocket client heartbeat failed, disconnecting.");
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }

    #[tracing::instrument(skip(self, ctx))]
    fn process_message(&self, msg: &str, ctx: &mut ws::WebsocketContext<WSSession>) {
        let addr = ctx.address();
        match WSMessage::from_str(msg) {
            Ok(msg) => match msg.task {
                message::WSTask::RoomConnect => todo!(),
            },
            Err(e) => {
                tracing::error!("{:?}", e);
                addr.do_send(e.into());
            }
        }
    }
}

impl Actor for WSSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WSSession {
    #[tracing::instrument(
        name = "Handling websocket message",
        skip_all,
        fields(message=tracing::field::Empty)
    )]
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match item {
            Ok(msg) => msg,
            Err(e) => {
                tracing::error!("Unexpected error: {:?}", e);
                ctx.stop();
                return;
            }
        };
        tracing::Span::current().record("message", &tracing::field::debug(&msg));

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => self.process_message(text.trim(), ctx),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {
                tracing::info!("Invalid message");
                ctx.stop();
            }
        }
    }
}
