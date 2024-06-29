use std::ops::DerefMut;

use anchor_lang::prelude::*;
use anchor_spl::token_2022::Token2022;

use crate::{check_season_ended, error::FomoLoveErrorCode, ConfigAccount, GameAccount, LeaderBoard, SeasonAccount, TopGame, UserAccount};
use solana_program::program::invoke_signed;

#[derive(Accounts)]
pub struct SubmitLeaderboard<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub config_account: Account<'info, ConfigAccount>,
    #[account(mut)]
    pub season_account: Account<'info, SeasonAccount>,
    #[account(mut)]
    pub game_account: Account<'info, GameAccount>,
    /// CHECK: Make sure the ata to the mint is actually owned by the signer
    #[account(mut)]
    pub nft_mint: AccountInfo<'info>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn submit_leaderboard(ctx: Context<SubmitLeaderboard>) -> Result<()> {

    let season_account = ctx.accounts.season_account.deref_mut();

    let game_account = &ctx.accounts.game_account;

    check_season_ended(season_account)?;

    let season_leaderboard = &mut season_account.leaderboard;

    require!(
        game_account.top_tile >= season_leaderboard.min_tile,
        FomoLoveErrorCode::LowTile
    );
    require!(
        game_account.score > season_leaderboard.min_score,
        FomoLoveErrorCode::LowScore
    );

    let top_game = TopGame {
        game: game_account.key(),
        player: ctx.accounts.user.key(),
        team: ctx.accounts.user_account.team,
        score: game_account.score,
        top_tile: game_account.top_tile,
    };

    update_leaderboard(season_leaderboard, top_game, season_leaderboard.max_game_count as usize)?;

    reset_game_account(&mut ctx.accounts.game_account);

    let seeds = b"config";
    let bump = ctx.accounts.config_account.bump;
    let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];

    invoke_signed(
        &spl_token_metadata_interface::instruction::update_field(
            &spl_token_2022::id(),
            ctx.accounts.nft_mint.to_account_info().key,
            ctx.accounts.config_account.to_account_info().key,
            spl_token_metadata_interface::state::Field::Key("game_over".to_string()),
            "true".to_string(),
        ),
        &[
            ctx.accounts.nft_mint.to_account_info().clone(),
            ctx.accounts.config_account.to_account_info().clone(),
        ],
        signer,
    )?;
    Ok(())
}

fn update_leaderboard(leaderboard: &mut LeaderBoard, top_game: TopGame, max_game_count: usize) -> Result<()> {
    let top_games = &mut leaderboard.top_games;

    top_games.push(top_game);

    top_games.sort_by(|a, b| {
        b.score.cmp(&a.score).then_with(|| b.top_tile.cmp(&a.top_tile))
    });

    // Limit quantity of games in leaderboard
    if top_games.len() > max_game_count {
        top_games.pop();
    }
    
    // Update min_score and min_tile if the leaderboard is not empty
    if top_games.len() == max_game_count {
        if let Some(bottom_game) = top_games.last() {
            leaderboard.min_score = bottom_game.score;
            leaderboard.min_tile = bottom_game.top_tile;
        }
    }

    Ok(())
}

fn reset_game_account(game_account: &mut GameAccount) {
    game_account.board = [[0; 4]; 4];
    game_account.score = 0;
    game_account.top_tile = 2;
}
