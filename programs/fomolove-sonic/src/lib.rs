use anchor_lang::prelude::*;

pub mod instructions;
pub mod states;
pub mod error;
pub mod utils;
pub mod constant;

use instructions::*;
use states::*;
use utils::*;
use constant::*;

declare_id!("GTUzkE8BaSyccP2MbG98MWDM7KE26SAAGGUTbByawTm8");

#[program]
pub mod fomolove_sonic {
    use super::*;   

    /* MAINTAINER FUNCTION */
    pub fn initialize(ctx: Context<Initialize>, chain_team_base_url: String, meme_team_base_url: String) -> Result<()> {
        instructions::initialize(ctx, chain_team_base_url, meme_team_base_url)?;
        Ok(())
    }

    pub fn update_season_duration(ctx: Context<UpdateSeasonDuration>, new_season_duration: u64) -> Result<()> {
        instructions::update_season_duration(ctx, new_season_duration)?;
        Ok(())
    }

    /* USER FUNCTION */
    pub fn start_season(ctx: Context<StartSeason>, start_time: u64) -> Result<()> {
        instructions::start_season(ctx,start_time)?;
        Ok(())
    }

    pub fn choose_team(ctx: Context<ChooseTeam>, team_type: TeamType) -> Result<()> {
        instructions::choose_team(ctx, team_type)?;
        Ok(())
    }

    pub fn register_game(ctx: Context<RegisterGame>) -> Result<()> {
        instructions::register_game(ctx)?;
        Ok(())
    }

    pub fn make_move(ctx: Context<MakeMove>, direction: Direction) -> Result<()> {
        instructions::make_move(ctx, direction)?;
        Ok(())
    }

    pub fn submit_leaderboard(ctx: Context<SubmitLeaderboard>) -> Result<()> {
        instructions::submit_leaderboard(ctx)?;
        Ok(())
    }
}
