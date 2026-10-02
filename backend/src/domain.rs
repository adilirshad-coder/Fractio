use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ProjectLifecycle { Draft, VerificationPending, Verified, SeedLive, SeedClosed, BondingLive, RaiseClosed, Graduating, Graduated, Failed, Refundable, Cancelled }
impl ProjectLifecycle {
    pub fn can_transition_to(self, next: Self) -> bool {
        use ProjectLifecycle::*;
        matches!((self, next), (Draft, VerificationPending) | (Draft, Cancelled) | (VerificationPending, Verified) | (VerificationPending, Cancelled) | (Verified, SeedLive) | (Verified, Cancelled) | (SeedLive, SeedClosed) | (SeedLive, Failed) | (SeedLive, Refundable) | (SeedLive, Cancelled) | (SeedClosed, BondingLive) | (SeedClosed, Failed) | (SeedClosed, Refundable) | (BondingLive, RaiseClosed) | (BondingLive, Failed) | (BondingLive, Refundable) | (RaiseClosed, Graduating) | (RaiseClosed, Failed) | (RaiseClosed, Refundable) | (Graduating, Graduated) | (Graduating, Failed) | (Failed, Refundable))
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum VerificationStage { Application, Documents, FinalReview }
impl VerificationStage {
    pub fn follows(previous: Option<Self>, next: Self) -> bool {
        matches!((previous, next), (None, Self::Application) | (Some(Self::Application), Self::Documents) | (Some(Self::Documents), Self::FinalReview))
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum KycLevel { Lite, Full }
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum KybStatus { None, Pending, Approved, Rejected }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lifecycle_transitions_match_contract() {
        assert!(ProjectLifecycle::Draft.can_transition_to(ProjectLifecycle::VerificationPending));
        assert!(ProjectLifecycle::Graduating.can_transition_to(ProjectLifecycle::Graduated));
        assert!(!ProjectLifecycle::Draft.can_transition_to(ProjectLifecycle::Graduated));
    }
    #[test]
    fn lifecycle_allowed_pairs_match_contract() {
        use ProjectLifecycle::*;
        let allowed = [
            (Draft, VerificationPending), (Draft, Cancelled),
            (VerificationPending, Verified), (VerificationPending, Cancelled),
            (Verified, SeedLive), (Verified, Cancelled),
            (SeedLive, SeedClosed), (SeedLive, Failed), (SeedLive, Refundable), (SeedLive, Cancelled),
            (SeedClosed, BondingLive), (SeedClosed, Failed), (SeedClosed, Refundable),
            (BondingLive, RaiseClosed), (BondingLive, Failed), (BondingLive, Refundable),
            (RaiseClosed, Graduating), (RaiseClosed, Failed), (RaiseClosed, Refundable),
            (Graduating, Graduated), (Graduating, Failed), (Failed, Refundable),
        ];
        for from in [Draft, VerificationPending, Verified, SeedLive, SeedClosed, BondingLive, RaiseClosed, Graduating, Graduated, Failed, Refundable, Cancelled] {
            for to in [Draft, VerificationPending, Verified, SeedLive, SeedClosed, BondingLive, RaiseClosed, Graduating, Graduated, Failed, Refundable, Cancelled] {
                assert_eq!(from.can_transition_to(to), allowed.contains(&(from, to)), "{from:?} -> {to:?}");
            }
        }
    }
    #[test]
    fn verification_stages_must_be_reviewed_in_order() {
        use VerificationStage::*;
        assert!(VerificationStage::follows(None, Application));
        assert!(VerificationStage::follows(Some(Application), Documents));
        assert!(VerificationStage::follows(Some(Documents), FinalReview));
        assert!(!VerificationStage::follows(None, Documents));
        assert!(!VerificationStage::follows(Some(Application), FinalReview));
        assert!(!VerificationStage::follows(Some(FinalReview), Application));
    }
}
