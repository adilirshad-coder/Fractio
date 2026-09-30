use serde::Deserialize;
#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub port: u16, pub database_url: String, pub database_max_connections: u32,
    pub redis_url: String, pub solana_rpc_url: String, pub solana_ws_url: String,
    pub solana_cluster: String, pub factory_program_id: String, pub seed_program_id: String,
    pub curve_program_id: String, pub capital_program_id: String, pub graduation_program_id: String,
    pub transfer_hook_program_id: String, pub jwt_secret: String, pub environment: String,
}
impl Settings {
    pub fn load() -> Result<Self, config::ConfigError> {
        let s: Self = config::Config::builder().add_source(config::Environment::default().try_parsing(true))
            .set_default("port", 8080)?.set_default("database_max_connections", 10)?
            .set_default("environment", "development")?.set_default("database_url", "postgres://fractio:fractio@localhost:5432/fractio")?
            .set_default("redis_url", "redis://localhost:6379")?.set_default("solana_rpc_url", "http://127.0.0.1:8899")?
            .set_default("solana_ws_url", "ws://127.0.0.1:8900")?.set_default("solana_cluster", "localnet")?
            .set_default("factory_program_id", "11111111111111111111111111111111")?.set_default("seed_program_id", "11111111111111111111111111111111")?
            .set_default("curve_program_id", "11111111111111111111111111111111")?.set_default("capital_program_id", "11111111111111111111111111111111")?
            .set_default("graduation_program_id", "11111111111111111111111111111111")?.set_default("transfer_hook_program_id", "11111111111111111111111111111111")?
            .set_default("jwt_secret", "local-development-only-change-this-secret")?.build()?.try_deserialize()?;
        if s.environment == "production" && s.jwt_secret.len() < 32 { return Err(config::ConfigError::Message("JWT_SECRET must have at least 32 characters in production".into())); }
        Ok(s)
    }
}
