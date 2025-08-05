

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction::transfer;
use crate::state::{Vault, Room};


#[derive(Accounts)]

pub struct InitializeVault<'info>{

    #[account(
        mut,
        seeds = [b"room",creator.key().as_ref()],
        bump,
        has_one = creator
    )]
    pub room : Account<'info,Room>,


    #[account(
        init,
        payer = creator,
        seeds = [b"vault",creator.key().as_ref(),room.key().as_ref()],
        space = 8 + Vault::INIT_SPACE,
        bump,

    )]
    pub vault : Account<'info,Vault>,

    #[account(mut)]
    pub creator : Signer<'info>,

    pub system_program : Program<'info,System>
}

pub fn handler(ctx : Context<InitializeVault>, amount : u64)-> Result<()>{
   
    let vault = &mut ctx.accounts.vault;
    let room = &mut ctx.accounts.room;

    let ix = transfer(&ctx.accounts.creator.key(), &vault.key(), amount);

    invoke(&ix, &[
        ctx.accounts.creator.to_account_info(),
        vault.to_account_info(),
    ])?;
    
    
    vault.room = room.key();
    vault.balance = amount;
    vault.vault_authority = ctx.accounts.creator.key();
    vault.bump = ctx.bumps.vault;
    vault.is_locked = true;
    
     msg!("Vault initialized with {} lamports", amount);

    Ok(())
}