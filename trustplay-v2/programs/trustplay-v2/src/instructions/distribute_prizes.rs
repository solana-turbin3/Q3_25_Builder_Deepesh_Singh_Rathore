
use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use crate::room::*;
use crate::vault_state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct DistributePrizes<'info> {
    #[account(mut)]
    pub organizer: Signer<'info>,

    #[account(
        seeds = [b"room", organizer.key().as_ref()],
        bump,
        constraint = room.organizer == organizer.key() @ ErrorCode::InvalidOrganizer
    )]
    pub room: Account<'info, Room>,

    #[account(
        seeds = [b"vault", room.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(mut)]
    pub winner_account : SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> DistributePrizes<'info> {
    pub fn distribute_prizes(&mut self) -> Result<()> {
        
        let room = &self.room;

        require!(room.players.contains(&self.winner_account.key()),ErrorCode::InvalidRecipient);

        let min_rent_for_vault_state = Rent::get()?.minimum_balance(self.vault_state.to_account_info().data_len());

        let distribution_prize = self.vault.lamports() - min_rent_for_vault_state;

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(self.system_program.to_account_info(), 
            Transfer{
                from : self.vault.to_account_info(),
                to : self.winner_account.to_account_info(),
            }, signer_seeds);

            transfer(cpi_ctx, distribution_prize)?;
        
     Ok(())
    }
}