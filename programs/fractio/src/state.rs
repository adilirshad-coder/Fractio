use anchor_lang::prelude::*;
#[account] #[derive(InitSpace)] pub struct GlobalConfig{pub authority:Pubkey,pub bump:u8,pub paused:bool}
#[account] #[derive(InitSpace)] pub struct Project{pub project_id:[u8;32],pub authority:Pubkey,pub status:ProjectStatus,pub allocation:SupplyAllocation,pub bump:u8}
#[account] #[derive(InitSpace)] pub struct Offering{pub project:Pubkey,pub status:OfferingStatus,pub bump:u8}
#[account] #[derive(InitSpace)] pub struct InvestorIdentity{pub reference_hash:[u8;32],pub eligible:bool,pub bump:u8}
#[account] #[derive(InitSpace)] pub struct InvestorPosition{pub project:Pubkey,pub identity:Pubkey,pub amount:u64,pub bump:u8}
#[account] #[derive(InitSpace)] pub struct SeedVesting{pub project:Pubkey,pub start_ts:i64,pub end_ts:i64,pub claimed:u64,pub bump:u8}
#[account] #[derive(InitSpace)] pub struct CurveState{pub project:Pubkey,pub virtual_quote:u128,pub virtual_base:u128,pub real_quote:u64,pub real_base:u64,pub bump:u8}
#[account] #[derive(InitSpace)] pub struct Milestone{pub project:Pubkey,pub index:u16,pub claimed:bool,pub bump:u8}
#[account] #[derive(InitSpace)] pub struct GraduationState{pub project:Pubkey,pub active:bool,pub seed_unlock_enabled:bool,pub bump:u8}
#[derive(AnchorSerialize,AnchorDeserialize,Clone,Copy,InitSpace,PartialEq,Eq)] pub struct SupplyAllocation{pub seed:u16,pub curve:u16,pub capital:u16,pub open_market:u16}
#[derive(AnchorSerialize,AnchorDeserialize,Clone,Copy,InitSpace,PartialEq,Eq)] pub enum ProjectStatus{Draft,VerificationPending,Verified,SeedLive,SeedClosed,BondingLive,RaiseClosed,Graduating,Graduated,Failed,Refundable,Cancelled}
#[derive(AnchorSerialize,AnchorDeserialize,Clone,Copy,InitSpace,PartialEq,Eq)] pub enum OfferingStatus{Draft,SeedLive,BondingLive,Closed,Failed,Refundable,Graduated}
#[event] pub struct GlobalConfigInitialized{pub authority:Pubkey}
#[event] pub struct ProjectCreated{pub project:Pubkey,pub authority:Pubkey}
#[event] pub struct SeedLaunched{pub project:Pubkey}
#[event] pub struct InvestmentRecorded{pub project:Pubkey,pub identity:Pubkey,pub amount:u64}
#[event] pub struct RefundRecorded{pub project:Pubkey,pub identity:Pubkey}
#[event] pub struct MilestoneClaimed{pub project:Pubkey,pub index:u16}
#[event] pub struct GraduationTriggered{pub project:Pubkey}
#[event] pub struct ProjectGraduated{pub project:Pubkey}
