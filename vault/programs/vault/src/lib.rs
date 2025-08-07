#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

declare_id!("7x4zQWLkLwMygSNYP3FwK172QpHTGirudn8TD5d8a3oY");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAccounts<'info> {

    #[account(mut)]
    pub user : Signer<'info>,


    #[account(
        init,
        payer = user,
        space = VaultState::INIT_SPACE,
        seeds = [b"vault" , user.key().as_ref()],
        bump
    )]
    pub vault_state : Account<'info,VaultState>,

    #[account(
        mut,
        seeds = [b"vault",vault_state.key().as_ref()],
        bump
    )]
    pub vault : SystemAccount<'info>,

    pub system_program : Program<'info,System>
}

impl<'info> InitializeAccounts<'info> {
    fn initialize(&mut self, bumps : &InitializeAccountsBumps)-> Result<()>{
        
        // we will need min amount to initialise the vault , min rent extdemption ( I don't know the spelling xD)
        let min_ammount: u64 = Rent::get()?.minimum_balance(self.vault.to_account_info().data_len());

        // creating a cpi context 
        
        
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct VaultState{
    vault_bump : u8,
    state_bump : u8
}