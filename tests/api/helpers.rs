use interactive_class::{
    configuration::get_configuration,
    telemetry::{get_subscriber, init_subscriber},
    Application,
};
use once_cell::sync::Lazy;
use reqwest::Response;

// Ensure that 'tracing' stack is only initialized once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub api_client: reqwest::Client,
}

impl TestApp {
    #[allow(unused)]
    pub async fn get_route(&self, route: &str) -> reqwest::Response {
        self.api_client
            .get(format!("{}/{}", &self.address, route))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn spawn_app() -> TestApp {
    // Set up tracing
    Lazy::force(&TRACING);

    // Randomise configuration to ensure test isolation
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Port 0 give us a random available port
        c.port = 0;
        c
    };

    // Launch app as background task
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    let application_port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let test_app = TestApp {
        address: format!("http://localhost:{}", application_port),
        port: application_port,
        api_client: client,
    };
    test_app
}

#[allow(unused)]
pub fn assert_is_redirect_to(response: &Response, location: &str) {
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), location);
}
