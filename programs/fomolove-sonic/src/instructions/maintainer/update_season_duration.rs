use anchor_lang::prelude::*;
use std::ops::DerefMut;

use crate::ConfigAccount;

#[derive(Accounts)]
pub struct UpdateSeasonDuration<'info> {
    #[account(mut)]
    pub maintainer: Signer<'info>,
    #[account(
        mut,
        constraint = maintainer.key() == config_account.maintainer.key()
    )]
    pub config_account: Account<'info, ConfigAccount>,
    pub system_program: Program<'info, System>,
}

pub fn update_season_duration(ctx: Context<UpdateSeasonDuration>, new_season_duration: u64) -> Result<()> {
    let config_account = ctx.accounts.config_account.deref_mut();
    config_account.season_duration = new_season_duration;
    Ok(())
}
