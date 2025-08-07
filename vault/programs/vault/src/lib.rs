#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL, system_program::{transfer, Transfer}};

declare_id!("7x4zQWLkLwMygSNYP3FwK172QpHTGirudn8TD5d8a3oY");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<InitializeAccounts>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;
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
        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(),Transfer{
            from : self.user.to_account_info(),
            to : self.vault.to_account_info()
        } );

        transfer(cpi_ctx, min_ammount)?;
        
        self.vault_state.state_bump = bumps.vault_state;
        self.vault_state.vault_bump = bumps.vault;


        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct VaultState{
    vault_bump : u8,
    state_bump : u8
}