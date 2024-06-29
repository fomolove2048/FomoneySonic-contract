use anchor_lang::prelude::*;

use crate::TeamType;

#[account]
pub struct UserAccount {
  pub bump: u8,
  pub team: TeamType,
}

impl Space for UserAccount {
    const INIT_SPACE: usize = 8 // Account discriminator added by Anchor for each account
        + 1 // bump
        + 1; // current_team

}