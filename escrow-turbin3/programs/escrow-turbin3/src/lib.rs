pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("CcC2szWmKyvsteJe1Z3yiD3JubQcxcb3o9tgFMkXzDWN");

#[program]
pub mod escrow_turbin3 {
    use super::*;

    pub fn make(ctx: Context<Make>,seeds: u64, amount: u64,) -> Result<()> {
        ctx.accounts.make(seeds,amount,&ctx.bumps)?;
        ctx.accounts.deposit(amount)
    }

    pub fn refund(ctx: Context<Refund>)-> Result<()>{
        ctx.accounts.refund()
    }

    pub fn take(ctx: Context<Take>)-> Result<()>{
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close()
    }
}
