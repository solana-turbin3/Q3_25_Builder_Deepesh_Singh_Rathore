use anchor_lang::prelude::*;
// creating one room for 

#[account]
#[derive(InitSpace)]

pub struct Room{
    #[max_len(50)]
    pub name : String,
    pub creator : Pubkey,
    pub player_count : u8,
    #[max_len(8)]
    pub players : Vec<Pubkey>
}