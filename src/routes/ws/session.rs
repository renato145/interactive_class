use super::{
    message::{ClientMessage, RoomInfo, WSMessage},
    ws,
};
use crate::{configuration::WSSettings, state::AppState};
use actix::{Actor, ActorContext, AsyncContext, Handler, StreamHandler};
use actix_web::web;
use std::{str::FromStr, time::Instant};

pub struct WSSession {
    hb: Instant,
    room: Option<String>,
    state: web::Data<AppState>,
    settings: WSSettings,
}

impl WSSession {
    pub fn new(state: web::Data<AppState>, settings: WSSettings) -> Self {
        Self {
            hb: Instant::now(),
            room: None,
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
    fn process_message(&mut self, msg: &str, ctx: &mut ws::WebsocketContext<WSSession>) {
        let addr = ctx.address();
        match WSMessage::from_str(msg) {
            Ok(msg) => match msg {
                WSMessage::RoomConnect(room_name) => addr.do_send(self.room_connect(room_name)),
            },
            Err(e) => {
                tracing::error!("{:?}", e);
                addr.do_send(e.into());
            }
        }
    }

    #[tracing::instrument(skip(self, ctx))]
    fn broadcast_message(&mut self, msg: &str, ctx: &mut ws::WebsocketContext<WSSession>) {
        tracing::error!("BROADCASTING {msg}");
    }

    fn get_room_info(&self) -> ClientMessage {
        match &self.room {
            Some(name) => match self.state.rooms.lock().unwrap().get(name) {
                Some(&connections) => ClientMessage::RoomInfo(RoomInfo {
                    name: name.clone(),
                    connections,
                }),
                None => {
                    tracing::error!("Invalid room on sessions: {name:?}");
                    ClientMessage::internal_error()
                }
            },
            None => ClientMessage::Error("No connected to any room".to_string()),
        }
    }

    /// Connects to a room and returns room information to the client
    #[tracing::instrument(skip(self))]
    fn room_connect(&mut self, room_name: String) -> ClientMessage {
        self.room = Some(room_name.clone());
        match self.state.rooms.lock().unwrap().get_mut(&room_name) {
            Some(connections) => {
                *connections += 1;
            }
            None => {
                return ClientMessage::Error(format!("Room not found {room_name:?}"));
            }
        };
        self.get_room_info()
    }
}

impl Actor for WSSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        self.broadcast_message("lalala", ctx);
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
