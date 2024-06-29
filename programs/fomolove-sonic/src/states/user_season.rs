use anchor_lang::prelude::*;

#[account]
pub struct UserSeasonAccount {
  pub bump: u8,
  pub season_id: u8,
  pub hightest_score: u32,
  pub game_played: u64,
}

impl Space for UserSeasonAccount {
    const INIT_SPACE: usize = 8 // Account discriminator added by Anchor for each account
        + 1 // bump
        + 1 // season_id
        + 4 // hightest score
        + 8; // game_played

}