use anchor_lang::prelude::*;

use crate::{error::FomoLoveErrorCode, SeasonAccount};

pub fn check_season_ended(season_account: &SeasonAccount) -> Result<()> {
    let now = Clock::get()?.unix_timestamp as u64;

    require!(
        season_account.started_at <= now && season_account.ended_at >= now,
        FomoLoveErrorCode::SeasonEnded
    );
    Ok(())
}
