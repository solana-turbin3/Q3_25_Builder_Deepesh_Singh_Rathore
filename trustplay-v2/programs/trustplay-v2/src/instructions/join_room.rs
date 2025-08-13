use anchor_lang::prelude::*;
use crate::room::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct JoinRoom<'info>{

    #[account(mut)]
    pub player : Signer<'info>,

    #[account(mut)]
    pub room : Account<'info,Room>,
    pub system_program : Program<'info,System>
}

impl<'info> JoinRoom<'info> {
    pub fn join_room(&mut self)->Result<()>{

        let room = &mut self.room;
        let player_key = self.player.key();

        if room.players.contains(&player_key){
            return err!(ErrorCode::PlayerAlreadyExist);
        }

        for slot in room.players.iter_mut(){
            if *slot == Pubkey::default(){
                *slot = player_key;
               return Ok(());
            }
        }
       
        Err(ErrorCode::RoomFull.into())
        

    }
}

