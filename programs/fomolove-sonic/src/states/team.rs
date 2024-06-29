use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TeamType {
    None,
    MemeTeam,
    ChainTeam,
}
#[account]
pub struct TeamAccount {
    pub bump: u8,
    pub team_type: TeamType,
    pub base_url: String,
    pub num_players: u32,
}

impl Space for TeamAccount {
    const INIT_SPACE: usize = 8 // Account discriminator added by Anchor for each account
        + 1 // bump
        + 1 // team_type
        + 8
        + 100; //base url
}
