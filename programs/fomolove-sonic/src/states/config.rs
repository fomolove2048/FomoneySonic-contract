use anchor_lang::prelude::*;

#[account]
pub struct ConfigAccount {
  pub bump: u8,
  pub is_initialized: bool,
  pub maintainer: Pubkey,
  pub current_season_id: u8,
  pub current_season_ended_at: u64,
  pub season_duration: u64
}

impl Space for ConfigAccount {
    const INIT_SPACE: usize = 8 // Account discriminator added by Anchor for each account
        + 1 // bump
        + 1 //is_initialized
        + 32 // maintainer
        + 1 //current_season_id
        + 8 // current_season_ended_at
        + 8; //season_duration
}