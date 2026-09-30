#![allow(dead_code)]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ProjectLifecycle { Draft, VerificationPending, Verified, SeedLive, SeedClosed, BondingLive, RaiseClosed, Graduating, Graduated, Failed, Refundable, Cancelled }
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum VerificationStage { Identity, Business, Project }
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ReviewStatus { Pending, Approved, Rejected, NeedsInformation, Expired }
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum FounderRole { SoleProprietor, Partner, Founder, CoFounder, AgenticFounder }
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct SupplyAllocationBps { pub seed: u16, pub bonding_curve: u16, pub capital_formation: u16, pub open_market: u16 }
impl SupplyAllocationBps { pub const STANDARD_LAUNCH: Self = Self { seed: 1_500, bonding_curve: 4_500, capital_formation: 1_500, open_market: 2_500 }; }
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GraduationPolicy { pub valuation_threshold_minor: Option<u128>, pub curve_allocation_sold_bps: Option<u16>, pub time_threshold: Option<DateTime<Utc>> }
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VestingSchedule { pub starts_at: DateTime<Utc>, pub ends_at: DateTime<Utc>, pub cadence_seconds: u64 }
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvestorIdentity { pub id: Uuid, pub provider_reference: Option<String>, pub eligibility: String, pub jurisdiction: Option<String>, pub expires_at: Option<DateTime<Utc>> }
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletAssociation { pub identity_id: Uuid, pub address: String, pub verified_at: Option<DateTime<Utc>> }
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FounderEntity { pub id: Uuid, pub entity_type: String, pub display_name: String }
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdempotencyRecord { pub key: String, pub request_hash: String, pub status: String }
