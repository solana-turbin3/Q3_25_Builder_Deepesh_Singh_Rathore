use anchor_lang::prelude::*;

use crate::room::Room;
use crate::participant::*;






#[derive(Accounts)]
pub struct InitializeRoom<'info>{

    // creating a PDA for ROOM created by creator.
    // seed should be , room , organiser pubkey

    #[account(
        init,
        space = 8 + Room::INIT_SPACE,
        payer = creator,
        seeds = [b"room",creator.key().as_ref()],
        bump
    )]

    pub room : Account<'info,Room>,

    #[account(mut)]
    pub creator : Signer<'info>,
    pub system_program : Program<'info,System>
}

#[derive(Accounts)]
pub struct JoinRoom<'info>{
    #[account(
        init,
        payer = participant,
        seeds = [b"participant", room.key().as_ref(),participant.key.as_ref()],
        space = 8 + Participant::INIT_SPACE,
        bump
    )]
    pub participantpda : Account<'info,Participant>,

    #[account(
        seeds = [b"room", room.creator.key().as_ref()],
        bump
    )]
    pub room : Account<'info,Room>,

    #[account(mut)]
    pub participant : Signer<'info>,

    pub system_program : Program<'info,System>
}