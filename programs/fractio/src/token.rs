//! Token-2022 integration boundary. Extension policy is deliberately not selected.
use anchor_lang::prelude::*;
pub struct TokenExtensionPolicy;
impl TokenExtensionPolicy { pub fn validate_program(_program:&Pubkey)->Result<()> { Ok(()) } }
