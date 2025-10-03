use zero2prod::{configuration::get_configuration, startup::Application, telemetry};

#[tokio::main]
async fn main() {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    tracing::info!("Starting process");

    let configuration = get_configuration().expect("Failed to read configuration");

    let application = Application::build(configuration)
        .await
        .expect("Failed to build 'Application' object");

    application
        .run_until_stopped()
        .await
        .expect("Failed to run appllication");
}
