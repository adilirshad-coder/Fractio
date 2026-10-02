use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GlobalConfig { pub authority: Pubkey, pub bump: u8, pub paused: bool }
#[account]
#[derive(InitSpace)]
pub struct Project { pub project_id: [u8; 32], pub authority: Pubkey, pub mint: Pubkey, pub total_supply: u64, pub status: ProjectStatus, pub allocation: SupplyAllocation, pub bump: u8 }
#[account]
#[derive(InitSpace)]
pub struct Offering { pub project: Pubkey, pub raise_target: u64, pub raise_deadline: i64, pub asset_mint: Pubkey, pub status: OfferingStatus, pub bump: u8 }
#[account]
#[derive(InitSpace)]
pub struct InvestorIdentity { pub reference_hash: [u8; 32], pub eligible: bool, pub bump: u8 }
#[account]
#[derive(InitSpace)]
pub struct InvestorPosition { pub project: Pubkey, pub identity: Pubkey, pub amount: u64, pub bump: u8 }
#[account]
#[derive(InitSpace)]
pub struct SeedVesting { pub project: Pubkey, pub start_ts: i64, pub end_ts: i64, pub claimed: u64, pub bump: u8 }
#[account]
#[derive(InitSpace)]
pub struct CurveState { pub project: Pubkey, pub virtual_quote: u128, pub virtual_base: u128, pub real_quote: u64, pub real_base: u64, pub bump: u8 }
#[account]
#[derive(InitSpace)]
pub struct Milestone { pub project: Pubkey, pub index: u16, pub claimed: bool, pub bump: u8 }
#[account]
#[derive(InitSpace)]
pub struct GraduationState { pub project: Pubkey, pub active: bool, pub seed_unlock_enabled: bool, pub bump: u8 }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace, PartialEq, Eq)]
pub struct SupplyAllocation { pub seed: u16, pub curve: u16, pub capital: u16, pub open_market: u16 }
impl SupplyAllocation {
    pub const STANDARD: Self = Self { seed: crate::constants::ALLOCATION_SEED_BPS, curve: crate::constants::ALLOCATION_CURVE_BPS, capital: crate::constants::ALLOCATION_CAPITAL_BPS, open_market: crate::constants::ALLOCATION_OPEN_MARKET_BPS };
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace, PartialEq, Eq, Debug)]
pub enum ProjectStatus { Draft, VerificationPending, Verified, SeedLive, SeedClosed, BondingLive, RaiseClosed, Graduating, Graduated, Failed, Refundable, Cancelled }
impl ProjectStatus {
    pub fn apply_submit(self) -> Result<Self> {
        require!(self == Self::Draft, crate::error::FractioError::InvalidProjectState);
        Ok(Self::VerificationPending)
    }

    pub fn apply_approve(self) -> Result<Self> {
        require!(self == Self::VerificationPending, crate::error::FractioError::InvalidProjectState);
        Ok(Self::Verified)
    }

    pub fn can_transition_to(self, next: Self) -> bool {
        use ProjectStatus::*;
        matches!((self, next), (Draft, VerificationPending) | (Draft, Cancelled) | (VerificationPending, Verified) | (VerificationPending, Cancelled) | (Verified, SeedLive) | (Verified, Cancelled) | (SeedLive, SeedClosed) | (SeedLive, Failed) | (SeedLive, Refundable) | (SeedLive, Cancelled) | (SeedClosed, BondingLive) | (SeedClosed, Failed) | (SeedClosed, Refundable) | (BondingLive, RaiseClosed) | (BondingLive, Failed) | (BondingLive, Refundable) | (RaiseClosed, Graduating) | (RaiseClosed, Failed) | (RaiseClosed, Refundable) | (Graduating, Graduated) | (Graduating, Failed) | (Failed, Refundable))
    }
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace, PartialEq, Eq, Debug)]
pub enum OfferingStatus { Draft, SeedLive, BondingLive, Closed, Failed, Refundable, Graduated }
impl OfferingStatus {
    pub fn accepts_investment(self) -> bool { matches!(self, Self::SeedLive | Self::BondingLive) }
}

#[event] pub struct GlobalConfigInitialized { pub authority: Pubkey }
#[event] pub struct ProjectCreated { pub project: Pubkey, pub authority: Pubkey }
#[event] pub struct ProjectApproved { pub project: Pubkey }
#[event] pub struct ProtocolPauseChanged { pub paused: bool }
#[event] pub struct SeedLaunched { pub project: Pubkey }
#[event] pub struct InvestmentRecorded { pub project: Pubkey, pub identity: Pubkey, pub amount: u64 }
#[event] pub struct RefundRecorded { pub project: Pubkey, pub identity: Pubkey }
#[event] pub struct MilestoneClaimed { pub project: Pubkey, pub index: u16 }
#[event] pub struct GraduationTriggered { pub project: Pubkey }
#[event] pub struct ProjectGraduated { pub project: Pubkey }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lifecycle_transitions_are_restricted() {
        assert!(ProjectStatus::Draft.can_transition_to(ProjectStatus::VerificationPending));
        assert!(ProjectStatus::Graduating.can_transition_to(ProjectStatus::Graduated));
        assert!(!ProjectStatus::Draft.can_transition_to(ProjectStatus::Graduated));
        assert!(!ProjectStatus::Graduated.can_transition_to(ProjectStatus::Failed));
    }
    #[test]
    fn lifecycle_allowed_pairs_match_contract() {
        use ProjectStatus::*;
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
    fn submit_and_approve_require_the_expected_states() {
        assert_eq!(ProjectStatus::Draft.apply_submit().unwrap(), ProjectStatus::VerificationPending);
        assert_eq!(ProjectStatus::VerificationPending.apply_approve().unwrap(), ProjectStatus::Verified);
        assert!(ProjectStatus::Draft.apply_approve().is_err());
        for status in [ProjectStatus::Draft, ProjectStatus::Verified, ProjectStatus::SeedLive, ProjectStatus::SeedClosed, ProjectStatus::BondingLive, ProjectStatus::RaiseClosed, ProjectStatus::Graduating, ProjectStatus::Graduated, ProjectStatus::Failed, ProjectStatus::Refundable, ProjectStatus::Cancelled] {
            assert!(status.apply_approve().is_err());
        }
        assert!(ProjectStatus::Verified.apply_submit().is_err());
    }
    #[test]
    fn every_account_has_init_space() {
        let sizes = [GlobalConfig::INIT_SPACE, Project::INIT_SPACE, Offering::INIT_SPACE, InvestorIdentity::INIT_SPACE, InvestorPosition::INIT_SPACE, SeedVesting::INIT_SPACE, CurveState::INIT_SPACE, Milestone::INIT_SPACE, GraduationState::INIT_SPACE];
        assert!(sizes.iter().all(|size| *size > 0));
    }
    #[test]
    fn offering_status_marks_only_live_phases_open() {
        assert!(OfferingStatus::SeedLive.accepts_investment());
        assert!(OfferingStatus::BondingLive.accepts_investment());
        assert!(!OfferingStatus::Closed.accepts_investment());
    }
}
