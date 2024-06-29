use anchor_lang::prelude::*;
use std::ops::DerefMut;

use crate::{error::FomoLoveErrorCode, ConfigAccount, TeamAccount, TeamType, WinnerAccount, DEFAULT_MAX_WINNER_COUNT, DEFAULT_SEASON_TIMER};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub maintainer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = maintainer,
        space = ConfigAccount::INIT_SPACE,
        seeds = [b"config".as_ref()],
        bump
    )]
    pub config_account: Account<'info, ConfigAccount>,
    #[account(
        init_if_needed,
        payer = maintainer,
        space = TeamAccount::INIT_SPACE,
        seeds = [b"meme_team".as_ref()],
        bump
      )]
    pub meme_team_account: Box<Account<'info, TeamAccount>>,
    #[account(
        init_if_needed,
        payer = maintainer,
        space = TeamAccount::INIT_SPACE,
        seeds = [b"chain_team".as_ref()],
        bump
      )]
    pub chain_team_account: Box<Account<'info, TeamAccount>>,
    #[account(
        init_if_needed,
        payer = maintainer,
        space = WinnerAccount::INIT_SPACE,
        seeds = [b"winner".as_ref()],
        bump
    )]
    pub winner_account: Box<Account<'info, WinnerAccount>>,
    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>, chain_team_base_url: String, meme_team_base_url: String) -> Result<()> {
    let config_account = ctx.accounts.config_account.deref_mut();
    let winner_account = ctx.accounts.winner_account.deref_mut();

    if config_account.is_initialized {
        return Err(FomoLoveErrorCode::AlreadyInitialized.into());
    }
    config_account.bump = ctx.bumps.config_account;

    config_account.maintainer = ctx.accounts.maintainer.key();
    config_account.current_season_id = 0;
    config_account.season_duration = DEFAULT_SEASON_TIMER;
    config_account.current_season_ended_at = Clock::get()?.unix_timestamp as u64;

    config_account.is_initialized = true; // Set the account as initialized

    //init winner account
    winner_account.bump = ctx.bumps.winner_account;
    winner_account.max_winner_count = DEFAULT_MAX_WINNER_COUNT;
    winner_account.leaderboard = Vec::new();

    //init meme team
    let meme_team = &mut ctx.accounts.meme_team_account;
    meme_team.bump = ctx.bumps.meme_team_account;
    meme_team.base_url = meme_team_base_url;
    meme_team.team_type = TeamType::MemeTeam;
    meme_team.num_players = 0;
    
    // Initialize the chain_team account
    let chain_team = &mut ctx.accounts.chain_team_account;
    chain_team.bump = ctx.bumps.chain_team_account;
    chain_team.team_type = TeamType::ChainTeam;
    chain_team.base_url = chain_team_base_url;
    chain_team.num_players = 0;

    Ok(())
}
