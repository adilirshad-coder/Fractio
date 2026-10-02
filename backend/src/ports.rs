use async_trait::async_trait;
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::RwLock;

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn roles_for(&self, subject: &str) -> anyhow::Result<Vec<String>>;
}

pub struct PostgresRoleRepository(pub sqlx::PgPool);
#[async_trait]
impl RoleRepository for PostgresRoleRepository {
    async fn roles_for(&self, subject: &str) -> anyhow::Result<Vec<String>> {
        Ok(sqlx::query_scalar::<_, String>("SELECT ur.role FROM user_roles ur JOIN users u ON u.id = ur.user_id WHERE u.auth_subject = $1")
            .bind(subject).fetch_all(&self.0).await?)
    }
}

#[derive(Default)]
pub struct MemoryRoleRepository(pub RwLock<HashMap<String, Vec<String>>>);
impl MemoryRoleRepository {
    pub fn set_roles(&self, subject: impl Into<String>, roles: Vec<String>) {
        self.0.write().expect("role repository lock").insert(subject.into(), roles);
    }
}
#[async_trait]
impl RoleRepository for MemoryRoleRepository {
    async fn roles_for(&self, subject: &str) -> anyhow::Result<Vec<String>> {
        Ok(self.0.read().expect("role repository lock").get(subject).cloned().unwrap_or_default())
    }
}
#[async_trait] pub trait ProjectRepository: Send + Sync { async fn find(&self, id: Uuid) -> anyhow::Result<Option<serde_json::Value>>; }
#[async_trait] pub trait ChainSubmitter: Send + Sync { async fn submit(&self, signed_tx: Vec<u8>, idempotency_key: &str) -> anyhow::Result<String>; }
#[async_trait]
pub trait ComplianceProvider: Send + Sync {
    async fn start_check(&self, subject_ref: &str, kind: &str) -> anyhow::Result<(String, String)>;
    async fn status(&self, external_reference: &str) -> anyhow::Result<String>;
}
#[async_trait] pub trait FxRateProvider: Send + Sync { async fn interbank_usd_pkr(&self) -> anyhow::Result<(u128, chrono::DateTime<chrono::Utc>)>; }
/// Implement with a Pyth or Switchboard feed adapter; no oracle policy is selected here.
#[async_trait] pub trait OracleProvider: Send + Sync { async fn price(&self, asset: &str) -> anyhow::Result<u128>; }
#[async_trait] pub trait PaymentProvider: Send + Sync { async fn create_intent(&self, key: &str, amount_minor: i64, currency: &str) -> anyhow::Result<String>; }
#[async_trait] pub trait VestingProvider: Send + Sync { async fn create_stream(&self, request: serde_json::Value, key: &str) -> anyhow::Result<String>; }
#[async_trait] pub trait OpenMarketAdapter: Send + Sync { async fn inspect_market(&self, project: Uuid) -> anyhow::Result<serde_json::Value>; }
#[async_trait] pub trait Signer: Send + Sync { async fn sign(&self, message: &[u8]) -> anyhow::Result<Vec<u8>>; }
#[async_trait] pub trait BondingCurveEngine: Send + Sync { async fn quote_buy(&self, project: Uuid, input: u128) -> anyhow::Result<u128>; async fn quote_sell(&self, project: Uuid, input: u128) -> anyhow::Result<u128>; }
