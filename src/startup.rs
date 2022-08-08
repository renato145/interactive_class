use crate::{
    configuration::{Settings, WSSettings},
    routes::{create_room, delete_room, get_cups_info, health_check_route, ws},
    state::AppState,
};
use actix_web::{dev::Server, web, App, HttpServer};
use anyhow::Result;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr()?.port();
        let server = run(listener, configuration.websocket).await?;
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

/// Index for the single web app
async fn spa_index() -> actix_files::NamedFile {
    actix_files::NamedFile::open("./frontend/dist/index.html").unwrap()
}

pub async fn run(listener: TcpListener, websocket_settings: WSSettings) -> Result<Server> {
    let websocket_settings = web::Data::new(websocket_settings);
    let app_state = web::Data::new(AppState::default());
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check_route))
            .route("/ws", web::get().to(ws))
            .service(
                web::scope("/cups")
                    .route("", web::get().to(get_cups_info))
                    .route("/create_room", web::post().to(create_room))
                    .route("/delete_room", web::delete().to(delete_room)),
            )
            .service(actix_files::Files::new("/", "./frontend/dist").index_file("index.html"))
            .default_service(web::get().to(spa_index))
            .app_data(websocket_settings.clone())
            .app_data(app_state.clone()) //
    })
    .listen(listener)?
    .run();
    Ok(server)
}
