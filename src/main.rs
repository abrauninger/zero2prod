use std::process::ExitCode;

use zero2prod::{configuration::get_configuration, startup::Application, telemetry};

#[tokio::main]
async fn main() -> ExitCode {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    tracing::info!("Starting process");

    if let Err(e) = execute().await {
        tracing::error!("Error during 'execute': {e:?}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

async fn execute() -> Result<(), anyhow::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
