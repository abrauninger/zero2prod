use zero2prod::{
    configuration::get_configuration, issue_delivery_worker::run_worker_until_stopped,
    startup::Application, telemetry,
};

#[tokio::main]
async fn main() {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    tracing::info!("Starting process");

    let backtrace_level = std::env::var("RUST_BACKTRACE")
        .ok()
        .unwrap_or("".to_string());

    if backtrace_level == "full" {
        tracing::info!(
            RUST_BACKTRACE = backtrace_level,
            "Full backtraces are enabled."
        );
    } else {
        tracing::warn!(
            RUST_BACKTRACE = backtrace_level,
            "RUST_BACKTRACE env var should be set to 'full' to enable proper diagnostics.",
        );
    };

    let configuration = get_configuration();

    let application = Application::build(configuration.clone()).await;
    let application_task = tokio::spawn(application.run_until_stopped());
    let worker_task = tokio::spawn(run_worker_until_stopped(configuration));

    tokio::select! {
        _ = application_task => {},
        _ = worker_task => {},
    };
}
