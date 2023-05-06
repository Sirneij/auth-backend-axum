#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    dotenv::from_filename(".env.development").ok();

    let settings = backend_axum::settings::get_settings().expect("Failed to read settings.");

    let subscriber = backend_axum::telemetry::get_subscriber(settings.clone().debug);
    backend_axum::telemetry::init_subscriber(subscriber);

    let app = backend_axum::startup::Application::build(settings, None).await?;

    tracing::event!(target: "backend_axum", tracing::Level::INFO, "Listening on http://127.0.0.1:{}/", app.port());

    Ok(())
}
