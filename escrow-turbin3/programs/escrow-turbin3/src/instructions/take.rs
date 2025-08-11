

use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{close_account, transfer_checked, CloseAccount, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::Escrow;

#[derive(Accounts)]
#[instruction(seeds : u64)]
pub struct Take<'info>{

    #[account(mut)]
    pub maker : SystemAccount<'info>,
    
    #[account(mut)]
    pub taker : Signer<'info>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_a : InterfaceAccount<'info,Mint>,
    
    #[account(
        mint::token_program = token_program
    )]

    pub mint_b : InterfaceAccount<'info,Mint>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_b: InterfaceAccount<'info,TokenAccount>,

    
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_a : InterfaceAccount<'info,TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_b : InterfaceAccount<'info,TokenAccount>,
    

    #[account(
        mut,
        close = maker, 
        has_one = maker,
        has_one = mint_a,
        has_one = mint_b,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seeds.to_le_bytes().as_ref()],
        bump = escrow.bump  
    )]
    pub escrow : Account<'info,Escrow>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault : InterfaceAccount<'info,TokenAccount>,


    pub system_program : Program<'info,System>,
    pub token_program : Interface<'info,TokenInterface>,
    pub associated_token_program : Program<'info,AssociatedToken>
    
}

impl<'info> Take<'info> {
    
    pub fn deposit(&mut self)-> Result<()>{

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), TransferChecked{
            from : self.taker_ata_b.to_account_info(),
            mint : self.mint_b.to_account_info(),
            to : self.maker_ata_b.to_account_info(),
            authority : self.taker.to_account_info()
        });

        transfer_checked(cpi_ctx, self.escrow.receive_amount, self.mint_b.decimals)
    }

    pub fn withdraw_and_close(&mut self)->Result<()>{

         let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seeds.to_le_bytes()[..],
            &[self.escrow.bump],
        ]];

        let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), TransferChecked{
            from : self.vault.to_account_info(),
            mint : self.mint_a.to_account_info(),
            to : self.taker_ata_a.to_account_info(),
            authority : self.taker.to_account_info()
        },&signer_seeds);

        transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)?;

        let cpi_close = CpiContext::new(self.token_program.to_account_info(), CloseAccount{
            account : self.vault.to_account_info(),
            destination : self.taker.to_account_info(),
            authority: self.escrow.to_account_info()
        });

        close_account(cpi_close)
    


    }
}
