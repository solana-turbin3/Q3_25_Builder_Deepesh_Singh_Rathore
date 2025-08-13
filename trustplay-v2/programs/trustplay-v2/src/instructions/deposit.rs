use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::room::*;
use crate::vault_state::*;

#[derive(Accounts)]
pub struct Deposit<'info>{

    #[account(mut)]
    pub organizer : Signer<'info>,

    #[account(
        seeds = [b"room",organizer.key().as_ref()],
        bump,
    )]
    pub room : Account<'info,Room>,

    
    #[account(
        seeds = [b"vault",room.key().as_ref()],
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

impl<'info> Deposit<'info>{
    
    pub fn deposit_to_vault(&mut self, amount : u64)-> Result<()>{


        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), Transfer{
            from : self.organizer.to_account_info(),
            to : self.vault.to_account_info()
        });
        transfer(cpi_ctx, amount)
        
    }

}
