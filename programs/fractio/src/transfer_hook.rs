//! Transfer-hook policy boundary; this scaffold does not authorize transfers.
use anchor_lang::prelude::*;
pub struct TransferValidation { pub source:Pubkey,pub destination:Pubkey,pub identity:Pubkey,pub project:Pubkey }
pub fn validate(_accounts:&TransferValidation)->Result<()> { // TODO: Verify identity, aggregate holding, phase, vesting and mint compatibility.
 err!(crate::error::FractioError::BusinessLogicNotImplemented)
}
