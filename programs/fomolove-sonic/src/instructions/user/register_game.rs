use std::ops::DerefMut;
use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::{self, AssociatedToken}, token_2022, token_interface::{spl_token_2022::instruction::AuthorityType, Token2022}
};
use solana_program::program::{invoke, invoke_signed};
use spl_token_2022::{extension::ExtensionType, state::Mint};

use crate::{
    check_season_ended, error::{FomoLoveErrorCode, ProgramErrorCode}, ConfigAccount, GameAccount, SeasonAccount, TeamType, UserAccount, UserSeasonAccount
};

#[derive(Accounts)]
pub struct RegisterGame<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub season_account: Account<'info, SeasonAccount>,
    #[account(mut)]
    pub nft_mint: Signer<'info>,
    #[account(
        init_if_needed,
        payer = user,
        space = UserSeasonAccount::INIT_SPACE,
        seeds = [b"user_season".as_ref(), &user.key().as_ref(), &[season_account.season_id]],
        bump
    )]
    pub user_season_account: Account<'info, UserSeasonAccount>,
    #[account(
      init_if_needed,
      payer = user,
      space = GameAccount::INIT_SPACE,
      seeds = [b"game".as_ref(), &nft_mint.key().as_ref()],
      bump
    )]
    pub game_account: Account<'info, GameAccount>,

    pub token_program: Program<'info, Token2022>,
    #[account(  
        mut
    )]
    pub config_account: Account<'info, ConfigAccount >,
    /// CHECK: We will create this one for the user
    #[account(mut)]
    pub token_account: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}

pub fn register_game(ctx: Context<RegisterGame>) -> Result<()> {
    let game = ctx.accounts.game_account.deref_mut();
    let season_account = ctx.accounts.season_account.deref_mut();
    let user_account = ctx.accounts.user_account.deref_mut();
    let user_season_account = ctx.accounts.user_season_account.deref_mut();

    check_season_ended(season_account)?;

    require!(
        user_account.team == TeamType::ChainTeam || user_account.team == TeamType::MemeTeam,
        FomoLoveErrorCode::NotChooseTeam
    );

    user_season_account.hightest_score = 0;
    user_season_account.game_played += 1;
    user_season_account.season_id = season_account.season_id;

    game.nft_mint = ctx.accounts.nft_mint.key();
    game.board = [[0; 4]; 4];
    game.score = 0;
    game.top_tile = 2;
    game.add_new_tile()?;
    game.add_new_tile()?;

    // Extract season_id before passing to the function to avoid borrowing issues
    let season_id = season_account.season_id;
    // Initialize mint and metadata
    initialize_mint_and_metadata(&ctx, season_id)?;

    Ok(())
}

pub fn initialize_mint_and_metadata(ctx: &Context<RegisterGame>, season_id: u8) -> Result<()> {
    let space = match ExtensionType::try_calculate_account_len::<Mint>(&[ExtensionType::MetadataPointer]) {
        Ok(space) => space,
        Err(_) => return err!(ProgramErrorCode::InvalidMintAccountSpace),
    };

    let meta_data_space = 250;
    let lamports_required = (Rent::get()?).minimum_balance(space + meta_data_space);

    // Create Mint account
    system_program::create_account(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            system_program::CreateAccount {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.nft_mint.to_account_info(),
            },
        ),
        lamports_required,
        space as u64,
        &ctx.accounts.token_program.key(),
    )?;

    // Assign the mint to the token program
    system_program::assign(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            system_program::Assign {
                account_to_assign: ctx.accounts.nft_mint.to_account_info(),
            },
        ),
        &token_2022::ID,
    )?;

    // Initialize the metadata pointer
    let init_meta_data_pointer_ix =
        match spl_token_2022::extension::metadata_pointer::instruction::initialize(
            &Token2022::id(),
            &ctx.accounts.nft_mint.key(),
            Some(ctx.accounts.config_account.key()),
            Some(ctx.accounts.nft_mint.key()),
        ) {
            Ok(ix) => ix,
            Err(_) => return err!(ProgramErrorCode::CantInitializeMetadataPointer),
        };

    invoke(
        &init_meta_data_pointer_ix,
        &[
            ctx.accounts.nft_mint.to_account_info(),
            ctx.accounts.config_account.to_account_info(),
        ],
    )?;

    // Initialize the mint cpi
    let mint_cpi_ix = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token_2022::InitializeMint2 {
            mint: ctx.accounts.nft_mint.to_account_info(),
        },
    );

    token_2022::initialize_mint2(
        mint_cpi_ix,
        0,
        &ctx.accounts.config_account.key(),
        None
    )?;

    // PDA for mint authority
    let seeds = b"config";
    let bump = ctx.accounts.config_account.bump;
    let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];

    let uri = if ctx.accounts.user_account.team == TeamType::MemeTeam {
        "https://bafybeidlf73itmw6hzskpy7amdcjzww3umwmvlwiqubbs2mkll2tnv7ojq.ipfs.nftstorage.link/me1.png".to_string()
    } else {
        "https://bafybeiferm3u2nsdnzcf25ubqdy3qjbn3bu6meeeue52lrkvlr3llqbpce.ipfs.nftstorage.link/chain1.png".to_string()
    };

    let init_token_meta_data_ix = &spl_token_metadata_interface::instruction::initialize(
        &spl_token_2022::id(),
        ctx.accounts.nft_mint.key,
        ctx.accounts.config_account.to_account_info().key,
        ctx.accounts.nft_mint.key,
        ctx.accounts.config_account.to_account_info().key,
        "Fomolove-Sonic".to_string(),
        "FLS".to_string(),
        uri,
    );

    invoke_signed(
        init_token_meta_data_ix,
        &[ctx.accounts.nft_mint.to_account_info().clone(), ctx.accounts.config_account.to_account_info().clone()],
        signer,
    )?;

    // Update the metadata account with an additional metadata field in this case the player level
    invoke_signed(
        &spl_token_metadata_interface::instruction::update_field(
            &spl_token_2022::id(),
            ctx.accounts.nft_mint.key,
            ctx.accounts.config_account.to_account_info().key,
            spl_token_metadata_interface::state::Field::Key("season".to_string()),
            season_id.to_string(),
        ),
        &[
            ctx.accounts.nft_mint.to_account_info().clone(),
            ctx.accounts.config_account.to_account_info().clone(),
        ],
        signer
    )?;

    invoke_signed(
        &spl_token_metadata_interface::instruction::update_field(
            &spl_token_2022::id(),
            ctx.accounts.nft_mint.key,
            ctx.accounts.config_account.to_account_info().key,
            spl_token_metadata_interface::state::Field::Key("game_over".to_string()),
            "false".to_string(),
        ),
        &[
            ctx.accounts.nft_mint.to_account_info().clone(),
            ctx.accounts.config_account.to_account_info().clone(),
        ],
        signer
    )?;

    // Create the associated token account
    associated_token::create(CpiContext::new(
        ctx.accounts.associated_token_program.to_account_info(),
        associated_token::Create {
            payer: ctx.accounts.user.to_account_info(),
            associated_token: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
            mint: ctx.accounts.nft_mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        },
    ))?;

    // Mint one token to the associated token account
    token_2022::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token_2022::MintTo {
                mint: ctx.accounts.nft_mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.config_account.to_account_info(),
            },
            signer,
        ),
        1,
    )?;

    // Freeze the mint authority to make it an NFT
    token_2022::set_authority(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token_2022::SetAuthority {
                current_authority: ctx.accounts.config_account.to_account_info(),
                account_or_mint: ctx.accounts.nft_mint.to_account_info(),
            },
            signer,
        ),
        AuthorityType::MintTokens,
        None,
    )?;

    Ok(())
}
