use anchor_lang::prelude::*;

#[error_code]
pub enum FomoLoveErrorCode {
    #[msg("Only authority can call this function")]
    Unauthorized,

    #[msg("The configuration account is already initialized.")]
    AlreadyInitialized,

    #[msg("Invalid team type")]
    InvalidTeamType,

    #[msg("Season ended")]
    SeasonEnded,

    #[msg("User already on team")]
    UserAlreadyOnTeam,

    #[msg("The game is not change.")]
    GameNotChange,

    #[msg("The game is over.")]
    GameOver,

    #[msg("The tile is too low.")]
    LowTile,
    
    #[msg("The score is too low.")]
    LowScore,

    #[msg("SeasonNotEnded")]
    SeasonNotEnded,

    #[msg("NotChooseTeam")]
    NotChooseTeam,

    #[msg("The team type in UserAccount does not match the team type in TeamAccount.")]
    TeamTypeMismatch,

    #[msg("Invalid Team")]
    InvalidTeam,
}

#[error_code]
pub enum ProgramErrorCode {
    #[msg("Invalid Mint account space")]
    InvalidMintAccountSpace,
    #[msg("Cant initialize metadata_pointer")]
    CantInitializeMetadataPointer,
}
