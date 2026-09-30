#![allow(dead_code)]
use async_trait::async_trait;
use uuid::Uuid;
#[async_trait] pub trait ProjectRepository: Send + Sync { async fn find(&self, id: Uuid) -> anyhow::Result<Option<serde_json::Value>>; }
#[async_trait] pub trait ChainSubmitter: Send + Sync { async fn submit(&self, signed_tx: Vec<u8>, idempotency_key: &str) -> anyhow::Result<String>; }
#[async_trait] pub trait ComplianceProvider: Send + Sync { async fn status(&self, external_reference: &str) -> anyhow::Result<String>; }
#[async_trait] pub trait PaymentProvider: Send + Sync { async fn create_intent(&self, key: &str, amount_minor: i64, currency: &str) -> anyhow::Result<String>; }
#[async_trait] pub trait VestingProvider: Send + Sync { async fn create_stream(&self, request: serde_json::Value, key: &str) -> anyhow::Result<String>; }
#[async_trait] pub trait OpenMarketAdapter: Send + Sync { async fn inspect_market(&self, project: Uuid) -> anyhow::Result<serde_json::Value>; }
#[async_trait] pub trait Signer: Send + Sync { async fn sign(&self, message: &[u8]) -> anyhow::Result<Vec<u8>>; }
#[async_trait] pub trait BondingCurveEngine: Send + Sync { async fn quote_buy(&self, project: Uuid, input: u128) -> anyhow::Result<u128>; async fn quote_sell(&self, project: Uuid, input: u128) -> anyhow::Result<u128>; }
