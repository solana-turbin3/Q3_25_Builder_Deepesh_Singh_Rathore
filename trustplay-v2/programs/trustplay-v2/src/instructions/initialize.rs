use anchor_lang::{prelude::*, system_program::{transfer, Transfer, create_account, CreateAccount}};


use crate::room::*;
use crate::vault_state::*;

#[derive(Accounts)]
pub struct InitializeRoom<'info>{

    #[account(mut)]
    pub organizer : Signer<'info>,

    #[account(
        init,
        payer = organizer,
        space = 8 + Room::INIT_SPACE,
        seeds = [b"room",organizer.key().as_ref()],
        bump,
    )]
    pub room : Account<'info,Room>,

    
    #[account(
        init,
        payer = organizer,
        space = 8 + VaultState::INIT_SPACE,
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

impl<'info> InitializeRoom<'info>{
    pub fn initialize(&mut self, bumps :&InitializeRoomBumps,tournament_name : String)-> Result<()>{

        let min_rent_for_room = Rent::get()?.minimum_balance(self.room.to_account_info().data_len());
        let min_rent_for_vault_state = Rent::get()?.minimum_balance(self.vault_state.to_account_info().data_len());

        let cpi_room = CpiContext::new(self.system_program.to_account_info(), Transfer{
            from : self.organizer.to_account_info(),
            to : self.room.to_account_info()
        });

        let cpi_vault  = CpiContext::new(self.system_program.to_account_info(), Transfer{
            from : self.organizer.to_account_info(),
            to : self.vault_state.to_account_info()
        });

        transfer(cpi_room, min_rent_for_room)?;
        transfer(cpi_vault, min_rent_for_vault_state)?;

        // Create the vault PDA as a system account (0 space) so it can hold SOL
        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[bumps.vault],
        ];
        let signer_seeds = &[&seeds[..]];

        let create_vault_ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            CreateAccount {
                from: self.organizer.to_account_info(),
                to: self.vault.to_account_info(),
            },
            signer_seeds,
        );
        // 0 lamports and 0 space is fine; deposits will fund it later
        create_account(create_vault_ctx, 0, 0, &System::id())?;
        
        self.room.set_inner(Room { tournament_name,organizer : self.organizer.key(), is_locked: false, players: [Pubkey::default();5], bump: bumps.room });
        msg!("Room Initialise");

        self.vault_state.set_inner(VaultState { vault_bump: bumps.vault,   state_bump: bumps.vault_state });
        msg!("Vault Initialized");
        Ok(())
    }
}
