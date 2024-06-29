use std::ops::DerefMut;

use anchor_lang::prelude::*;

use crate::{error::FomoLoveErrorCode, ConfigAccount, LeaderBoard, SeasonAccount, DEFAULT_LEADERBOARD_MAX_GAME_COUNT, DEFAULT_LEADERBOARD_MIN_SCORE, DEFAULT_LEADERBOARD_MIN_TILE};

#[derive(Accounts)]
pub struct StartSeason<'info> {
    #[account(mut)]
    pub maintainer: Signer<'info>,
    #[account(
        mut,
        constraint = config_account.maintainer == maintainer.key()
    )]
    pub config_account: Account<'info, ConfigAccount>,
    #[account(
      init_if_needed,
      payer = maintainer,
      space = SeasonAccount::INIT_SPACE,
      seeds = [b"season".as_ref(), &[config_account.current_season_id + 1]],
      bump
    )]
    pub season_account: Account<'info, SeasonAccount>,
    pub system_program: Program<'info, System>,
}

pub fn start_season(ctx: Context<StartSeason>, start_time: u64) -> Result<()> {
    let config_account = ctx.accounts.config_account.deref_mut();

    let season_account = ctx.accounts.season_account.deref_mut();
    
    let now = Clock::get()?.unix_timestamp as u64;

    require!(start_time >= config_account.current_season_ended_at && now >= config_account.current_season_ended_at, FomoLoveErrorCode::SeasonNotEnded);

    // Initialize the season account
    season_account.bump = ctx.bumps.season_account;
    season_account.started_at = start_time;
    season_account.ended_at = start_time + config_account.season_duration;
    season_account.season_id = config_account.current_season_id + 1;
    season_account.leaderboard = LeaderBoard {
        min_score: DEFAULT_LEADERBOARD_MIN_SCORE,
        min_tile: DEFAULT_LEADERBOARD_MIN_TILE,
        top_games: Vec::new(),
        max_game_count: DEFAULT_LEADERBOARD_MAX_GAME_COUNT,
    };

    config_account.current_season_id = config_account.current_season_id + 1;
    config_account.current_season_ended_at = start_time + config_account.season_duration;

    Ok(())
}
