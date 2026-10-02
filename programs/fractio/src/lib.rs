use anchor_lang::prelude::*;
declare_id!("2ZwVTBRrzyTmzm5UJSoPjuvJYr1cr9qWkzWEqJq5WFBN");
pub mod constants;
pub mod error;
pub mod state;
pub mod token;
pub mod transfer_hook;
use error::FractioError;
use state::*;

#[program]
pub mod fractio {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let c = &mut ctx.accounts.config;
        c.authority = ctx.accounts.authority.key();
        c.bump = ctx.bumps.config;
        c.paused = true;
        emit!(GlobalConfigInitialized { authority: c.authority });
        Ok(())
    }
    pub fn create_project(
        ctx: Context<CreateProject>,
        project_id: [u8; 32],
        total_supply: u64
    ) -> Result<()> {
        require!(!ctx.accounts.config.paused, FractioError::Paused);
        require!(constants::valid_total_supply(total_supply), FractioError::InvalidSupply);
        let p = &mut ctx.accounts.project;
        p.project_id = project_id;
        p.authority = ctx.accounts.authority.key();
        p.mint = Pubkey::default();
        p.total_supply = total_supply;
        p.status = ProjectStatus::Draft;
        p.allocation = SupplyAllocation::STANDARD;
        p.bump = ctx.bumps.project;
        emit!(ProjectCreated { project: p.key(), authority: p.authority });
        Ok(())
    }
    pub fn approve_project(ctx: Context<AdminAction>) -> Result<()> {
        require!(!ctx.accounts.config.paused, FractioError::Paused);
        ctx.accounts.project.status = ctx.accounts.project.status.apply_approve()?;
        emit!(ProjectApproved { project: ctx.accounts.project.key() });
        Ok(())
    }
    pub fn submit_for_verification(ctx: Context<FounderAction>) -> Result<()> {
        require!(!ctx.accounts.config.paused, FractioError::Paused);
        ctx.accounts.project.status = ctx.accounts.project.status.apply_submit()?;
        emit!(ProjectSubmitted { project: ctx.accounts.project.key() });
        Ok(())
    }
    pub fn set_paused(ctx: Context<AdminConfigAction>, paused: bool) -> Result<()> {
        ctx.accounts.config.paused = paused;
        emit!(ProtocolPauseChanged { paused });
        Ok(())
    }
    pub fn launch_seed(ctx: Context<FounderAction>) -> Result<()> {
        require!(!ctx.accounts.config.paused, FractioError::Paused);
        require!(ctx.accounts.project.status == ProjectStatus::Verified, FractioError::InvalidProjectState);
        err!(FractioError::BusinessLogicNotImplemented)
    }
    pub fn buy(ctx: Context<InvestorAction>, _amount: u64, _max_payment: u64) -> Result<()> {
        require!(!ctx.accounts.config.paused, FractioError::Paused);
        require!(ctx.accounts.project.status == ProjectStatus::BondingLive, FractioError::InvalidProjectState);
        // TODO(decision): curve and settlement behavior remain intentionally unimplemented.
        err!(FractioError::BusinessLogicNotImplemented)
    }
    pub fn refund(ctx: Context<InvestorAction>) -> Result<()> {
        require!(!ctx.accounts.config.paused, FractioError::Paused);
        require!(
            matches!(ctx.accounts.project.status, ProjectStatus::Refundable | ProjectStatus::Failed),
            FractioError::InvalidProjectState
        );
        err!(FractioError::BusinessLogicNotImplemented)
    }
    pub fn claim_milestone(ctx: Context<FounderAction>, _index: u16) -> Result<()> {
        require!(!ctx.accounts.config.paused, FractioError::Paused);
        // TODO(decision): milestone slabs are claimed during bonding, not only after RaiseClosed.
        require!(ctx.accounts.project.status == ProjectStatus::RaiseClosed, FractioError::InvalidProjectState);
        err!(FractioError::BusinessLogicNotImplemented)
    }
    pub fn graduate(ctx: Context<InvestorAction>) -> Result<()> {
        require!(!ctx.accounts.config.paused, FractioError::Paused);
        require!(ctx.accounts.project.status == ProjectStatus::RaiseClosed, FractioError::InvalidProjectState);
        err!(FractioError::BusinessLogicNotImplemented)
    }
}

#[event]
pub struct ProjectSubmitted { pub project: Pubkey }

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)] pub authority: Signer<'info>,
    #[account(
        constraint = program.programdata_address()? == Some(program_data.key()),
        constraint = program_data.upgrade_authority_address == Some(authority.key())
    )] pub program_data: Account<'info, ProgramData>,
    pub program: Program<'info, crate::program::Fractio>,
    #[account(
        init,
        payer = authority,
        space = 8 + GlobalConfig::INIT_SPACE,
        seeds = [constants::GLOBAL_SEED],
        bump
    )] pub config: Account<'info, GlobalConfig>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
#[instruction(project_id: [u8; 32], total_supply: u64)]
pub struct CreateProject<'info> {
    #[account(mut)] pub authority: Signer<'info>,
    #[account(seeds = [constants::GLOBAL_SEED], bump = config.bump)] pub config: Account<
        'info,
        GlobalConfig
    >,
    #[account(
        init,
        payer = authority,
        space = 8 + Project::INIT_SPACE,
        seeds = [constants::PROJECT_SEED, project_id.as_ref()],
        bump
    )] pub project: Account<'info, Project>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct InvestorAction<'info> {
    pub investor: Signer<'info>,
    #[account(seeds = [constants::GLOBAL_SEED], bump = config.bump)] pub config: Account<
        'info,
        GlobalConfig
    >,
    #[account(mut, seeds=[constants::PROJECT_SEED, project.project_id.as_ref()], bump=project.bump)] pub project: Account<
        'info,
        Project
    >,
}
#[derive(Accounts)]
pub struct FounderAction<'info> {
    pub authority: Signer<'info>,
    #[account(seeds = [constants::GLOBAL_SEED], bump = config.bump)] pub config: Account<
        'info,
        GlobalConfig
    >,
    #[account(mut, has_one=authority, seeds=[constants::PROJECT_SEED, project.project_id.as_ref()], bump=project.bump)] pub project: Account<
        'info,
        Project
    >,
}
#[derive(Accounts)]
pub struct AdminAction<'info> {
    pub authority: Signer<'info>,
    #[account(mut, seeds=[constants::GLOBAL_SEED], bump=config.bump, has_one=authority)] pub config: Account<
        'info,
        GlobalConfig
    >,
    #[account(mut, seeds=[constants::PROJECT_SEED, project.project_id.as_ref()], bump=project.bump)] pub project: Account<
        'info,
        Project
    >,
}
#[derive(Accounts)]
pub struct AdminConfigAction<'info> {
    pub authority: Signer<'info>,
    #[account(mut, seeds=[constants::GLOBAL_SEED], bump=config.bump, has_one=authority)] pub config: Account<
        'info,
        GlobalConfig
    >,
}
