mod api;
mod config;
mod domain;
mod errors;
mod ports;

use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry().with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "fractio_backend=info".into())).with(tracing_subscriber::fmt::layer().json()).init();
    let settings = config::Settings::load()?;
    let db = PgPoolOptions::new().max_connections(settings.database_max_connections).connect(&settings.database_url).await?;
    if std::env::args().any(|arg| arg == "migrate") { sqlx::migrate!("../migrations").run(&db).await?; return Ok(()); }
    let app = api::router(api::AppState { db, settings: settings.clone() });
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", settings.port)).await?;
    tracing::info!(port = settings.port, "Fractio backend listening");
    axum::serve(listener, app).await?;
    Ok(())
}
