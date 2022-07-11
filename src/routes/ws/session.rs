use super::{
    message::{ClientMessage, WSMessage, WSTask},
    ws,
};
use crate::{configuration::WSSettings, state::AppState};
use actix::{Actor, ActorContext, AsyncContext, Handler, StreamHandler};
use actix_web::web;
use std::{str::FromStr, time::Instant};

pub struct WSSession {
    hb: Instant,
    state: web::Data<AppState>,
    settings: WSSettings,
}

impl WSSession {
    pub fn new(state: web::Data<AppState>, settings: WSSettings) -> Self {
        Self {
            hb: Instant::now(),
            state,
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
            }
            ctx.ping(b"");
        });
    }

    #[tracing::instrument(skip(self, ctx))]
    fn process_message(&self, msg: &str, ctx: &mut ws::WebsocketContext<WSSession>) {
        let addr = ctx.address();
        match WSMessage::from_str(msg) {
            Ok(msg) => match msg.task {
                WSTask::RoomConnect => addr.do_send(self.room_connect()),
            },
            Err(e) => {
                tracing::error!("{:?}", e);
                addr.do_send(e.into());
            }
        }
        println!("{:?}", self.state);
    }

    fn room_connect(&self) -> ClientMessage {
        // let mut rooms = self.state.rooms.lock().unwrap();
        // rooms.push("Something".to_string());
        ClientMessage::success()
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

impl Handler<ClientMessage> for WSSession {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) -> Self::Result {
        match serde_json::to_string(&msg) {
            Ok(msg) => ctx.text(msg),
            Err(e) => {
                tracing::error!(error.cause_chain = ?e, error.message = %e, "Failed to send ClientMessage.")
            }
        }
    }
}
