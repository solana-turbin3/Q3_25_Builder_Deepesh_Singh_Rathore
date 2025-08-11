use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{transfer_checked, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::Escrow;

#[derive(Accounts)]
#[instruction(seeds : u64)]
pub struct Make<'info>{

    #[account(mut)]
    pub maker : Signer<'info>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_a : InterfaceAccount<'info,Mint>,
    
    #[account(
        mint::token_program = token_program
    )]

    pub mint_b : InterfaceAccount<'info,Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a : InterfaceAccount<'info,TokenAccount>,
    
    #[account(
        init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", maker.key().as_ref(), seeds.to_le_bytes().as_ref()],
        bump
        
    )]
    pub escrow : Account<'info,Escrow>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault : InterfaceAccount<'info,TokenAccount>,


    pub system_program : Program<'info,System>,
    pub token_program : Interface<'info,TokenInterface>,
    pub associated_token_program : Program<'info,AssociatedToken>
    
}

impl<'info> Make<'info> {
    
    pub fn make(&mut self, seeds : u64, amount : u64, bumps : &MakeBumps)-> Result<()>{

        self.escrow.set_inner(Escrow { 
            seeds,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            receive_amount: amount, 
            bump: bumps.escrow 
        });

        Ok(())
    }

    pub fn deposite(&mut self, amount : u64)-> Result<()>{

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), TransferChecked{
            from : self.maker_ata_a.to_account_info(),
            mint : self.mint_a.to_account_info(),
            to : self.vault.to_account_info(),
            authority : self.maker.to_account_info()
        });

        transfer_checked(cpi_ctx, amount, self.mint_a.decimals)
    }
}
