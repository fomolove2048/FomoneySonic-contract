use anchor_lang::prelude::*;

use crate::{TeamType, DEFAULT_MAX_WINNER_COUNT};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct WinnerTopGame {
    pub user: Pubkey,
    pub team: TeamType,
    pub season_id: u8,
    pub score: u32,
    pub top_tile: u16,
}

#[account]
pub struct WinnerAccount {
  pub bump: u8,
  pub max_winner_count: u8,
  pub leaderboard: Vec<WinnerTopGame>,
}

impl Space for WinnerAccount {
  const INIT_SPACE: usize = 8 // Account discriminator added by Anchor for each account
      + 1 // bump
      + 1 // max_winner_count
      + 4 // length of the leaderboard vector
      + DEFAULT_MAX_WINNER_COUNT as usize * (
            32 // user
          + 1
          + 1 // season_id
          + 4 // score
          + 2 // top_tile
      );
}
