use crate::{
    configuration::{Settings, WSSettings},
    routes::{health_check_route, ws},
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
        let server = run(
            listener,
            configuration.application.base_url,
            configuration.websocket,
        )
        .await?;
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub struct ApplicationBaseUrl(pub String);

pub async fn run(
    listener: TcpListener,
    base_url: String,
    websocket_settings: WSSettings,
) -> Result<Server> {
    let base_url = web::Data::new(ApplicationBaseUrl(base_url));
    let websocket_settings = web::Data::new(websocket_settings);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            // .route("/", web::get().to(home))
            .route("/health_check", web::get().to(health_check_route))
            .route("/ws", web::get().to(ws))
            // .service(actix_files::Files::new("/static", "./static"))
            // .default_service(web::get().to(not_found))
            .app_data(base_url.clone())
            .app_data(websocket_settings.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
