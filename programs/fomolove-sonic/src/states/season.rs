use anchor_lang::prelude::*;

use crate::{TeamType, DEFAULT_LEADERBOARD_MAX_GAME_COUNT};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct TopGame {
    pub game: Pubkey,
    pub player: Pubkey,
    pub team: TeamType,
    pub score: u32,
    pub top_tile: u16, // Updated to u16
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LeaderBoard {
    pub min_tile: u16, // Updated to u16
    pub max_game_count: u8,
    pub min_score: u32,
    pub top_games: Vec<TopGame>,
}

#[account]
pub struct SeasonAccount {
    pub bump: u8,
    pub season_id: u8,
    pub started_at: u64,
    pub ended_at: u64,
    pub total_game_played: u64,
    pub leaderboard: LeaderBoard,
}

impl Space for SeasonAccount {
    const INIT_SPACE: usize = 8 // Account discriminator added by Anchor for each account
        + 1 // bump
        + 1 // season_id
        + 8 // started_at
        + 8 // ended_at
        + 8 // total_game_played
        + 2 // min_tile (updated to u16, hence 2 bytes)
        + 1 // max_game_count
        + 4 // min_score
        // top_games
        + 4 + DEFAULT_LEADERBOARD_MAX_GAME_COUNT as usize * (
            32 // game
            + 32 // player
            + 4 // score
            + 1 //team type
            + 2 // top_tile (updated to u16, hence 2 bytes)
        ); // TopGame
}
