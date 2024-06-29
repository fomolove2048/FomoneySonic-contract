use std::ops::DerefMut;

use crate::{
    error::FomoLoveErrorCode, ConfigAccount, Direction, GameAccount, TeamAccount, UserAccount,
    UserSeasonAccount, WinnerAccount, WinnerTopGame,
};
use anchor_lang::prelude::*;
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::{Mint, TokenAccount};
use solana_program::program::invoke_signed;

#[derive(Accounts)]
pub struct MakeMove<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub config_account: Account<'info, ConfigAccount>,
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub winner_account: Account<'info, WinnerAccount>,
    #[account(mut)]
    pub user_team_account: Account<'info, TeamAccount>,
    #[account(mut)]
    pub user_season_account: Account<'info, UserSeasonAccount>,
    #[account(mut,
        constraint = game.nft_mint.key() == nft_mint.key()
    )]
    pub game: Account<'info, GameAccount>,
    #[account(mut)]
    pub nft_mint: InterfaceAccount<'info, Mint>,
    #[account(mut,
        token::mint = nft_mint.key(),
        token::authority = user.key(),
    )]
    pub nft_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,

    pub system_program: Program<'info, System>,
}

pub fn make_move(ctx: Context<MakeMove>, direction: Direction) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let config_account = &ctx.accounts.config_account;
    let user_account = &ctx.accounts.user_account;
    let user_team_account = &ctx.accounts.user_team_account;

    let winner_account = ctx.accounts.winner_account.deref_mut();

    let user_season_account = ctx.accounts.user_season_account.deref_mut();

    let old_board = game.board;

    let old_top_tile = game.top_tile; // Store the old value of top_tile

    // Check if team types match
    if user_account.team != user_team_account.team_type {
        return Err(FomoLoveErrorCode::TeamTypeMismatch.into());
    }
    match direction {
        Direction::Up => game.move_up(),
        Direction::Down => game.move_down(),
        Direction::Left => game.move_left(),
        Direction::Right => game.move_right(),
    }

    if old_board == game.board {
        return Err(FomoLoveErrorCode::GameNotChange.into());
    }

    if game.is_game_over() {
        return Err(FomoLoveErrorCode::GameOver.into());
    }

    game.add_new_tile()?;

    // Update the highest score if the current game's score is higher
    if game.score > user_season_account.hightest_score {
        user_season_account.hightest_score = game.score;
    }

    // Check if top_tile has been updated
    if game.top_tile != old_top_tile {
        let seeds = b"config";
        let bump = ctx.accounts.config_account.bump;
        let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];

        let base_url = &user_team_account.base_url;
        let suffix = map_top_tile_to_suffix(game.top_tile);
        let new_url = format!("{}{}.png", base_url, suffix);

        invoke_signed(
            &spl_token_metadata_interface::instruction::update_field(
                &spl_token_2022::id(),
                ctx.accounts.nft_mint.to_account_info().key,
                ctx.accounts.config_account.to_account_info().key,
                spl_token_metadata_interface::state::Field::Uri,
                new_url.to_string(),
            ),
            &[
                ctx.accounts.nft_mint.to_account_info().clone(),
                ctx.accounts.config_account.to_account_info().clone(),
            ],
            signer,
        )?;
    }

    // Check if top_tile reaches 2048 and update WinnerAccount
    if game.top_tile >= 2048 {
        let user_pubkey = ctx.accounts.user.key();
        let winner_top_game = WinnerTopGame {
            user: user_pubkey,
            team: user_account.team,
            season_id: config_account.current_season_id,
            score: game.score,
            top_tile: game.top_tile,
        };

        let user_exists = winner_account
            .leaderboard
            .iter()
            .position(|entry| entry.user == user_pubkey);

        match user_exists {
            Some(index) => {
                let entry = &mut winner_account.leaderboard[index];
                if winner_top_game.score > entry.score
                    || (winner_top_game.score == entry.score
                        && winner_top_game.top_tile > entry.top_tile)
                {
                    winner_account.leaderboard[index] = winner_top_game;
                }
            }
            None => {
                winner_account.leaderboard.push(winner_top_game);
            }
        }

        if winner_account.leaderboard.len() > winner_account.max_winner_count as usize {
            winner_account.leaderboard.sort_by(|a, b| {
                b.score
                    .cmp(&a.score)
                    .then_with(|| b.top_tile.cmp(&a.top_tile))
            });
            winner_account
                .leaderboard
                .truncate(winner_account.max_winner_count as usize);
        }
    }

    Ok(())
}

fn map_top_tile_to_suffix(top_tile: u16) -> String {
    match top_tile {
        2 => "1".to_string(),
        4 => "2".to_string(),
        8 => "3".to_string(),
        16 => "4".to_string(),
        32 => "5".to_string(),
        64 => "6".to_string(),
        128 => "7".to_string(),
        256 => "8".to_string(),
        512 => "9".to_string(),
        1024 => "10".to_string(),
        2048 => "11".to_string(),
        _ => "1".to_string(),
    }
}
