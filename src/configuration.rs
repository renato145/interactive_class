use config::Config;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_with::{serde_as, DurationMilliSeconds};
use std::time::Duration;
#[derive(Clone, Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub websocket: WSSettings,
}

#[derive(Clone, Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[serde_as]
#[derive(Clone, Deserialize)]
pub struct WSSettings {
    /// In milliseconds
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub heartbeat_interval: Duration,
    /// In milliseconds
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub client_timeout: Duration,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory.");
    let configuration_directory = base_path.join("configuration");

    // Detect the running environment.
    // Default to `local` if unspecified.
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    let mut settings = Config::builder()
        // Read the "default" configuration file
        .add_source(config::File::from(configuration_directory.join("base")).required(true))
        // Layer on the environment-specific values.
        .add_source(
            config::File::from(configuration_directory.join(environment.as_str())).required(true),
        )
        // Add in settings from environment variables (with a prefix of APP and '__' as separator)
        // E.g. `APP__APPLICATION__PORT=5001` would set `Settings.application.port`
        .add_source(config::Environment::with_prefix("app").separator("__"))
        .build()?
        .try_deserialize::<Settings>()?;

    // For deploying to heroku we need to read the PORT environment variable
    if let Ok(Ok(port)) = std::env::var("PORT").map(|x| x.parse()) {
        settings.application.port = port;
    }

    Ok(settings)
}

/// The possible runtime environment for our application.
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}
