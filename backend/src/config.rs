use serde::Deserialize;

pub const FRACTIO_PROGRAM_ID: &str = "2ZwVTBRrzyTmzm5UJSoPjuvJYr1cr9qWkzWEqJq5WFBN";

#[derive(Clone, Debug, Deserialize)]
#[allow(dead_code)] // Settings are kept configurable before all scaffold routes consume them.
pub struct Settings {
    pub port: u16,
    pub database_url: String,
    pub database_max_connections: u32,
    pub redis_url: String,
    pub solana_rpc_url: String,
    pub solana_ws_url: String,
    pub solana_cluster: String,
    pub fractio_program_id: String,
    pub jwt_secret: String,
    pub jwt_algorithm: String,
    pub jwt_public_key: String,
    pub jwt_issuer: String,
    pub jwt_audience: String,
    pub kyc_provider: String,
    pub environment: String,
    pub ownership_cap_bps: u16,
    pub total_supply: u64,
    pub allocation_seed_bps: u16,
    pub allocation_curve_bps: u16,
    pub allocation_capital_bps: u16,
    pub allocation_open_market_bps: u16,
    pub seed_vesting_days: u16,
    pub milestone_cap_pkr: u64,
    pub pkr_fx_source: String,
}
impl Settings {
    pub fn load() -> Result<Self, config::ConfigError> {
        let s: Self = config::Config::builder()
            .add_source(config::Environment::default().try_parsing(true))
            .set_default("port", 8080)?.set_default("database_max_connections", 10)?
            .set_default("environment", "development")?
            .set_default("database_url", "postgres://fractio:fractio@localhost:5432/fractio")?
            .set_default("redis_url", "redis://localhost:6379")?
            .set_default("solana_rpc_url", "http://127.0.0.1:8899")?
            .set_default("solana_ws_url", "ws://127.0.0.1:8900")?
            .set_default("solana_cluster", "localnet")?
            .set_default("fractio_program_id", FRACTIO_PROGRAM_ID)?
            .set_default("jwt_secret", "local-development-only-change-this-secret")?
            .set_default("jwt_algorithm", "HS256")?.set_default("jwt_public_key", "")?
            .set_default("jwt_issuer", "")?.set_default("jwt_audience", "")?
            .set_default("kyc_provider", "mock")?
            .set_default("ownership_cap_bps", 500)?.set_default("total_supply", 100_000_000_u64)?
            .set_default("allocation_seed_bps", 1500)?.set_default("allocation_curve_bps", 4500)?
            .set_default("allocation_capital_bps", 1500)?.set_default("allocation_open_market_bps", 2500)?
            .set_default("seed_vesting_days", 365)?.set_default("milestone_cap_pkr", 50_000_000_u64)?
            .set_default("pkr_fx_source", "interbank")?
            .build()?.try_deserialize()?;
        if !valid_total_supply(s.total_supply) {
            return Err(config::ConfigError::Message("TOTAL_SUPPLY must be 100000000 or 1000000000".into()));
        }
        if !valid_allocation(s.allocation_seed_bps, s.allocation_curve_bps, s.allocation_capital_bps, s.allocation_open_market_bps) { return Err(config::ConfigError::Message("allocation basis points must sum to 10000".into())); }
        if s.ownership_cap_bps == 0 || s.ownership_cap_bps > 10_000 || s.seed_vesting_days == 0 || s.milestone_cap_pkr == 0 {
            return Err(config::ConfigError::Message("ownership cap must be 1..=10000 bps; vesting days and milestone cap must be positive".into()));
        }
        if !matches!(s.jwt_algorithm.as_str(), "HS256" | "RS256" | "ES256" | "EdDSA") {
            return Err(config::ConfigError::Message("JWT_ALGORITHM must be HS256, RS256, ES256, or EdDSA".into()));
        }
        if !matches!(s.kyc_provider.as_str(), "mock" | "none") {
            return Err(config::ConfigError::Message("KYC_PROVIDER must be mock or none".into()));
        }
        if s.environment == "production" && s.kyc_provider == "mock" {
            return Err(config::ConfigError::Message("KYC_PROVIDER=mock is forbidden in production".into()));
        }
        if s.environment == "production" && (s.jwt_algorithm == "HS256" || s.jwt_secret.len() < 32 || s.jwt_secret == "local-development-only-change-this-secret") {
            return Err(config::ConfigError::Message("Production requires asymmetric JWT verification; HS256 and short secrets are not allowed".into()));
        }
        if s.environment == "production" && s.jwt_public_key.is_empty() {
            return Err(config::ConfigError::Message("JWT_PUBLIC_KEY is required for asymmetric verification".into()));
        }
        if s.environment == "production" && (s.jwt_issuer.is_empty() || s.jwt_audience.is_empty()) {
            return Err(config::ConfigError::Message("JWT_ISSUER and JWT_AUDIENCE are required in production".into()));
        }
        Ok(s)
    }
}
fn valid_total_supply(total: u64) -> bool { matches!(total, 100_000_000 | 1_000_000_000) }
fn valid_allocation(seed: u16, curve: u16, capital: u16, open_market: u16) -> bool {
    u32::from(seed) + u32::from(curve) + u32::from(capital) + u32::from(open_market) == 10_000
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validates_supply_presets_and_allocation_total() {
        assert!(valid_total_supply(100_000_000));
        assert!(valid_total_supply(1_000_000_000));
        assert!(!valid_total_supply(42));
        assert!(valid_allocation(1500, 4500, 1500, 2500));
        assert!(!valid_allocation(1500, 4500, 1500, 1500));
    }
}
