use interactive_class::{
    configuration::get_configuration,
    telemetry::{get_subscriber, init_subscriber},
    Application,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("interactive_class".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration.clone()).await?;
    application.run_until_stopped().await?;
    Ok(())
}
