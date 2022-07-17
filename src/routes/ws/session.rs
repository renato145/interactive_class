use super::{
    error::WSError,
    message::{ClientMessage, ConnectionType, CupColor, RoomConnectInfo, WSMessage},
    ws,
};
use crate::{configuration::WSSettings, state::AppState};
use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, StreamHandler};
use actix_web::web;
use std::{str::FromStr, time::Instant};
use uuid::Uuid;

pub struct WSSession {
    id: Uuid,
    hb: Instant,
    room: Option<String>,
    state: web::Data<AppState>,
    settings: WSSettings,
}

impl WSSession {
    pub fn new(state: web::Data<AppState>, settings: WSSettings) -> Self {
        Self {
            id: Uuid::new_v4(),
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
    fn process_message(&mut self, message: &str, ctx: &mut ws::WebsocketContext<WSSession>) {
        let addr = ctx.address();
        match WSMessage::from_str(message) {
            Ok(msg) => match msg {
                WSMessage::RoomConnect(room_info) => {
                    self.room_connect(room_info, addr);
                }
                WSMessage::ChooseCup(color) => {
                    self.choose_cup(color, addr);
                }
            },
            Err(e) => {
                tracing::error!(error.cause_chain =? e, error.message = %e, "Failed to parse message.");
                addr.do_send(e.into());
            }
        }
    }

    #[tracing::instrument(skip(self))]
    fn broadcast_message(&self, message: ClientMessage, connection_type: ConnectionType) {
        match &self.room {
            Some(name) => match self.state.rooms.lock().unwrap().get(name) {
                Some(room_state) => {
                    let connections = match connection_type {
                        ConnectionType::Student => &room_state.student_connections,
                        ConnectionType::Teacher => &room_state.teacher_connections,
                    };

                    connections
                        .iter()
                        .filter(|&(id, _)| id != &self.id)
                        .for_each(|(_, addr)| {
                            addr.do_send(message.clone());
                        })
                }
                None => {
                    tracing::warn!(error.message = %WSError::InvalidRoom(name.clone()));
                }
            },
            None => {
                tracing::warn!(error.message = %WSError::NoRoom);
            }
        }
    }

    #[tracing::instrument(skip(self))]
    fn broadcast_all(&self, message: ClientMessage) {
        self.broadcast_message(message.clone(), ConnectionType::Teacher);
        self.broadcast_message(message, ConnectionType::Student);
    }

    fn get_room_info(&self) -> ClientMessage {
        match &self.room {
            Some(name) => match self.state.rooms.lock().unwrap().get(name) {
                Some(room_state) => ClientMessage::RoomInfo(room_state.clone().into()),
                None => {
                    let e = WSError::InvalidRoom(name.clone());
                    tracing::warn!(error.message = %e);
                    e.into()
                }
            },
            None => WSError::NoRoom.into(),
        }
    }

    /// Connects to a room, returns room information to the client and
    /// broadcast information to teachers
    #[tracing::instrument(skip(self, addr))]
    fn room_connect(&mut self, room_info: RoomConnectInfo, addr: Addr<Self>) {
        let room_name = room_info.room_name;
        self.room = Some(room_name.clone());
        let msg = match self.state.rooms.lock().unwrap().get_mut(&room_name) {
            Some(room_state) => {
                let connections = match room_info.connection_type {
                    ConnectionType::Student => &mut room_state.student_connections,
                    ConnectionType::Teacher => &mut room_state.teacher_connections,
                };
                if connections
                    .insert(self.id, addr.clone().recipient())
                    .is_none()
                {
                    Ok(())
                } else {
                    Err(WSError::AlreadyConnected)
                }
            }
            None => Err(WSError::InvalidRoom(room_name)),
        };
        match msg {
            Ok(_) => {
                let msg = self.get_room_info();
                addr.do_send(msg.clone());
                if let ConnectionType::Student = room_info.connection_type {
                    self.broadcast_message(msg, ConnectionType::Teacher);
                }
            }
            Err(msg) => {
                addr.do_send(msg.into());
            }
        }
    }

    /// Student chooses a cup color and broadcast information to teachers
    #[tracing::instrument(skip(self, addr))]
    fn choose_cup(&mut self, color: CupColor, addr: Addr<Self>) {
        let msg = match &self.room {
            Some(room) => match self.state.rooms.lock().unwrap().get_mut(room) {
                Some(room_state) => {
                    room_state.add_cup(color);
                    ClientMessage::Ok
                }
                None => WSError::InvalidRoom(room.clone()).into(),
            },
            None => WSError::NoRoom.into(),
        };
        addr.do_send(msg);
        self.broadcast_message(self.get_room_info(), ConnectionType::Teacher);
    }
}

impl Actor for WSSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        match &self.room {
            Some(name) => match self.state.rooms.lock().unwrap().get_mut(name) {
                Some(room_state) => {
                    if room_state.student_connections.remove(&self.id).is_none() {
                        tracing::warn!(error.message = %WSError::InvalidRoom(name.clone()), "Couldn't remove session.");
                    }
                }
                None => {
                    tracing::warn!(error.message = %WSError::InvalidRoom(name.clone()));
                }
            },
            None => {
                tracing::warn!(error.message = %WSError::NoRoom);
            }
        }
        let msg = self.get_room_info();
        self.broadcast_all(msg);
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
                tracing::error!(error.cause_chain =? e, error.message = %e, "Unexpected error.");
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
