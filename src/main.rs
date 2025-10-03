use zero2prod::{configuration::get_configuration, startup::Application, telemetry};

#[tokio::main]
async fn main() {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    tracing::info!("Starting process");

    let backtrace_level = std::env::var("RUST_BACKTRACE").ok();
    let backtrace_setting = match &backtrace_level {
        Some(value) => format!("Running with 'RUST_BACKTRACE={value}'."),
        None => "Running without 'RUST_BACKTRACE' set.".to_string(),
    };

    if let Some(level) = &backtrace_level
        && level == "full"
    {
        tracing::info!("{backtrace_setting}");
    } else {
        tracing::warn!(
            "{backtrace_setting} RUST_BACKTRACE env var should be set to 'full' to enable proper diagnostics."
        );
    };

    let configuration = get_configuration().expect("Failed to read configuration");

    let application = Application::build(configuration).await;

    application
        .run_until_stopped()
        .await
        .expect("Failed to run appllication");
}
