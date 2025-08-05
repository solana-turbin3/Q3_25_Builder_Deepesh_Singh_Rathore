
use anchor_lang::prelude::*;
use crate::context::*;
use crate::constants::*;
use crate::error::JoinRoomError;

pub fn handler(ctx:Context<JoinRoom>)->Result<()>{

    let room = &mut ctx.accounts.room;
    let participant = &mut ctx.accounts.participantpda;

    require!(room.player_count < MAX_PLAYER,
    JoinRoomError::RoomFull);


    participant.room = room.key();
    participant.participant_player = ctx.accounts.participant.key();

    room.players.push(ctx.accounts.participant.key());

    room.player_count += 1;

    msg!("Participated in room :{} \n Participant:{}",participant.room,participant.participant_player);
    Ok(())
}