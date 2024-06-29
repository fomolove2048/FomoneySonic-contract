use std::ops::DerefMut;

use anchor_lang::prelude::*;

use crate::{error::FomoLoveErrorCode, TeamAccount, TeamType, UserAccount};

#[derive(Accounts)]
pub struct ChooseTeam<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub team_meme_account: Account<'info, TeamAccount>,
    #[account(mut)]
    pub team_chain_account: Account<'info, TeamAccount>,
    #[account(
        init_if_needed,
        payer = user,
        space = UserAccount::INIT_SPACE,
        seeds = [b"user".as_ref(), &user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,

    pub system_program: Program<'info, System>,
}

pub fn choose_team(ctx: Context<ChooseTeam>, team_type: TeamType) -> Result<()> {
    let team_meme_account = ctx.accounts.team_meme_account.deref_mut();
    let team_chain_account = ctx.accounts.team_chain_account.deref_mut();

    let user_account = ctx.accounts.user_account.deref_mut();

    match user_account.team {
        TeamType::None => {},
        _ => return Err(FomoLoveErrorCode::UserAlreadyOnTeam.into()),
    }

    match team_type {
        TeamType::MemeTeam => {
            team_meme_account.num_players += 1;
        }
        TeamType::ChainTeam => {
            team_chain_account.num_players += 1;
        }
        TeamType::None => return Err(FomoLoveErrorCode::InvalidTeam.into()),
    }
    //handle user
    user_account.team = team_type;

    Ok(())
}
