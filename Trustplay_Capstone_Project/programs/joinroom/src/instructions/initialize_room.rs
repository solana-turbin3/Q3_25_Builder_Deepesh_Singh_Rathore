use anchor_lang::prelude::*;
use crate::context::*;



pub fn handler(ctx: Context<InitializeRoom>, name : String) -> Result<()> {
    let room = &mut ctx.accounts.room;

    room.name = name;
    room.creator = ctx.accounts.creator.key();
    msg!("Room created \n Room Name :{} \n Room Creator:{}",room.name,room.creator);
    Ok(())
}
