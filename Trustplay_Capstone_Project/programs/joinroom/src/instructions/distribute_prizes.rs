use anchor_lang::{accounts::program, prelude::*, solana_program::{lamports, program::{invoke, invoke_signed}, system_instruction::transfer}};

use crate::{vault, Room, Vault};


#[derive(Accounts)]
pub struct DistributePrize<'info>{

    #[account(
        seeds = [b"room", creator.key().as_ref()],
        bump,
        has_one = creator
    )]
    pub room : Account<'info,Room>,

    #[account(
        mut,
        seeds = [b"vault", creator.key().as_ref(),room.key().as_ref()],
        bump = vault.bump
    )]

    pub vault : Account<'info,Vault>,

    #[account(mut)]
    pub creator : Signer<'info>,



}

impl<'info> DistributePrize<'info> {
  pub fn distribute_prizes(&mut self,remaining: &'info [AccountInfo<'info>],)->Result<()>{

    let vault = &self.vault;

    let vault_lamports = vault.get_lamports();
    let remaining_accounts = remaining;

    let prize_per_winner = vault_lamports / remaining_accounts.len() as u64;

    for winner in remaining_accounts.iter(){

        let ix = transfer(&vault.key(), &winner.key(), prize_per_winner);

        invoke_signed(
            &ix,
            &[
                vault.to_account_info(),
                winner.clone(), // Must clone because of borrowing
            ],
            &[&[
                b"vault",
                self.creator.key.as_ref(),
                self.room.key().as_ref(),
                &[self.vault.bump],
            ]],
        )?;


    }

Ok(())

}
}
