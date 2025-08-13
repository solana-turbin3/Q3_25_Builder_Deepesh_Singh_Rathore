use anchor_lang:: prelude::*;

#[account]
#[derive(InitSpace)]

pub struct Room{
    #[max_len(50)]
    pub tournament_name : String,
    pub organizer : Pubkey,

    pub is_locked : bool,
    pub players : [Pubkey; 5],
    pub bump : u8
}