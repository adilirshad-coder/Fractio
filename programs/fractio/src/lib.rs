use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWxTWqkZ7FEfcYkgMQHGqjf2Kk4Y");
pub mod constants; pub mod error; pub mod state; pub mod token; pub mod transfer_hook;
use error::FractioError; use state::*;
#[program]
pub mod fractio {
 use super::*;
 pub fn initialize(ctx:Context<Initialize>)->Result<()> { let c=&mut ctx.accounts.config; c.authority=ctx.accounts.authority.key(); c.bump=ctx.bumps.config; c.paused=true; emit!(GlobalConfigInitialized{authority:c.authority}); Ok(()) }
 pub fn create_project(ctx:Context<CreateProject>, project_id:[u8;32], allocation:SupplyAllocation)->Result<()> { require!(allocation.seed as u32+allocation.curve as u32+allocation.capital as u32+allocation.open_market as u32==10_000,FractioError::InvalidAllocation); let p=&mut ctx.accounts.project; p.project_id=project_id;p.authority=ctx.accounts.authority.key();p.status=ProjectStatus::Draft;p.allocation=allocation;p.bump=ctx.bumps.project;emit!(ProjectCreated{project:p.key(),authority:p.authority});Ok(()) }
 pub fn launch_seed(_ctx:Context<ProjectAction>)->Result<()> {
  // TODO: Implement verified transition checks.
  err!(FractioError::BusinessLogicNotImplemented)
 }
 pub fn buy(_ctx:Context<ProjectAction>,_amount:u64,_max_payment:u64)->Result<()> {
  // TODO: Implement integer curve math and settlement.
  err!(FractioError::BusinessLogicNotImplemented)
 }
 pub fn refund(_ctx:Context<ProjectAction>)->Result<()> {
  // TODO: Implement finalized refundable-state settlement.
  err!(FractioError::BusinessLogicNotImplemented)
 }
 pub fn claim_milestone(_ctx:Context<ProjectAction>,_index:u16)->Result<()> {
  // TODO: Implement milestone authorization and settlement.
  err!(FractioError::BusinessLogicNotImplemented)
 }
 pub fn graduate(_ctx:Context<ProjectAction>)->Result<()> {
  // TODO: Implement configured graduation and AMM handoff.
  err!(FractioError::BusinessLogicNotImplemented)
 }
}
#[derive(Accounts)] pub struct Initialize<'info>{#[account(mut)]pub authority:Signer<'info>,#[account(init,payer=authority,space=8+GlobalConfig::INIT_SPACE,seeds=[constants::GLOBAL_SEED],bump)]pub config:Account<'info,GlobalConfig>,pub system_program:Program<'info,System>}
#[derive(Accounts)] #[instruction(project_id:[u8;32],allocation:SupplyAllocation)] pub struct CreateProject<'info>{#[account(mut)]pub authority:Signer<'info>,#[account(seeds=[constants::GLOBAL_SEED],bump=config.bump)]pub config:Account<'info,GlobalConfig>,#[account(init,payer=authority,space=8+Project::INIT_SPACE,seeds=[constants::PROJECT_SEED,project_id.as_ref()],bump)]pub project:Account<'info,Project>,pub system_program:Program<'info,System>}
#[derive(Accounts)] pub struct ProjectAction<'info>{pub authority:Signer<'info>,#[account(mut,has_one=authority,seeds=[constants::PROJECT_SEED,project.project_id.as_ref()],bump=project.bump)]pub project:Account<'info,Project>}
