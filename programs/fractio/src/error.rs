use anchor_lang::prelude::*;
#[error_code] pub enum FractioError {
 #[msg("Protocol is paused")] Paused,
 #[msg("Invalid lifecycle state")] InvalidProjectState,
 #[msg("Caller is not authorized")] Unauthorized,
 #[msg("Investor is not eligible")] InvestorNotEligible,
 #[msg("Ownership cap would be exceeded")] OwnershipCapExceeded,
 #[msg("Allocation basis points must sum to 10000")] InvalidAllocation,
 #[msg("Invalid mint or token program")] InvalidMint,
 #[msg("Invalid vault or PDA")] InvalidVault,
 #[msg("Slippage limit exceeded")] SlippageExceeded,
 #[msg("Graduation condition is not met")] GraduationNotReady,
 #[msg("Milestone condition is not met or was already claimed")] MilestoneNotClaimable,
 #[msg("Checked arithmetic overflow or underflow")] MathError,
 #[msg("Business logic remains intentionally unimplemented")] BusinessLogicNotImplemented,
}
