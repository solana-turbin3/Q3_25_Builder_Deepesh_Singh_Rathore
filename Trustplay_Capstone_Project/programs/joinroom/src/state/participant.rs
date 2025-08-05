use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Participant{
    pub participant_player : Pubkey,
    pub room : Pubkey,
}