mod api;
#[allow(dead_code)]
mod adapters;
mod auth;
mod config;
#[allow(dead_code)] // Domain contracts are scaffolded ahead of their service handlers.
mod domain;
mod errors;
#[allow(dead_code)] // These ports intentionally remain unimplemented integration boundaries.
mod ports;

use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry().with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "fractio_backend=info".into())).with(tracing_subscriber::fmt::layer().json()).init();
    let settings = config::Settings::load()?;
    let db = PgPoolOptions::new().max_connections(settings.database_max_connections).connect(&settings.database_url).await?;
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("migrate") => { sqlx::migrate!("../migrations").run(&db).await?; return Ok(()); }
        Some("grant-role") => {
            let subject = option(&args, "--subject")?;
            let role = option(&args, "--role")?;
            anyhow::ensure!(["investor", "founder", "admin"].contains(&role.as_str()), "role must be investor, founder, or admin");
            let reason = optional_option(&args, "--reason");
            let mut tx = db.begin().await?;
            sqlx::query("INSERT INTO users(auth_subject) VALUES ($1) ON CONFLICT(auth_subject) DO NOTHING").bind(&subject).execute(&mut *tx).await?;
            let user_id: uuid::Uuid = sqlx::query_scalar("SELECT id FROM users WHERE auth_subject=$1").bind(&subject).fetch_one(&mut *tx).await?;
            sqlx::query("INSERT INTO user_roles(user_id,role,grant_reason) VALUES ($1,$2,$3) ON CONFLICT(user_id,role) DO UPDATE SET grant_reason=EXCLUDED.grant_reason")
                .bind(user_id).bind(role).bind(reason).execute(&mut *tx).await?;
            tx.commit().await?;
            println!("Role granted.");
            return Ok(());
        }
        Some("dev-token") => {
            anyhow::ensure!(settings.environment != "production", "dev-token is disabled in production");
            anyhow::ensure!(settings.jwt_algorithm == "HS256", "dev-token requires JWT_ALGORITHM=HS256");
            #[derive(serde::Serialize)] struct Claims { sub: String, exp: usize }
            let subject = option(&args, "--sub")?;
            let minutes: usize = optional_option(&args, "--minutes").unwrap_or_else(|| "60".into()).parse()?;
            anyhow::ensure!(minutes > 0 && minutes <= 24 * 60, "minutes must be between 1 and 1440");
            let exp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs() as usize + minutes * 60;
            let token = jsonwebtoken::encode(&jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256), &Claims { sub: subject, exp }, &jsonwebtoken::EncodingKey::from_secret(settings.jwt_secret.as_bytes()))?;
            println!("{token}");
            return Ok(());
        }
        _ => {}
    }
    let role_repository = std::sync::Arc::new(ports::PostgresRoleRepository(db.clone()));
    let app = api::router(api::AppState { db, settings: settings.clone(), roles: role_repository });
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", settings.port)).await?;
    tracing::info!(port = settings.port, "Fractio backend listening");
    axum::serve(listener, app).await?;
    Ok(())
}

fn option(args: &[String], name: &str) -> anyhow::Result<String> {
    optional_option(args, name).ok_or_else(|| anyhow::anyhow!("missing required argument {name}"))
}

fn optional_option(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|arg| arg == name).and_then(|index| args.get(index + 1)).cloned()
}
