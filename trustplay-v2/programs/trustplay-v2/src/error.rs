use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Player already exists in the room")]
    PlayerAlreadyExist,
    #[msg("Room is full")]
    RoomFull,
    #[msg("Invalid amount: must be greater than 0")]
    InvalidAmount,
    #[msg("Invalid Recipient")]
    InvalidRecipient,
    #[msg("Invalid organizer")]
    InvalidOrganizer,
    #[msg("No winners provided")]
    NoWinners,
    #[msg("Too many winners (maximum 3)")]
    TooManyWinners,
    #[msg("Vault is empty")]
    EmptyVault,
    #[msg("Insufficient prize amount")]
    InsufficientPrize,
    #[msg("Winner is not in the room")]
    WinnerNotInRoom,
    #[msg("Number of winner accounts does not match number of winners")]
    WinnersAccountsMismatch,
    #[msg("Provided winner account does not match the winner public key")]
    WinnerAccountMismatch,
}
