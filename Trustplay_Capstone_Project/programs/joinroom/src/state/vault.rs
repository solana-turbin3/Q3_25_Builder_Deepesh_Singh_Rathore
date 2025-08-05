use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]

pub struct Vault {
    pub room : Pubkey,
    pub balance : u64,
    pub vault_authority : Pubkey,
    pub is_locked : bool,
    pub bump : u8
}