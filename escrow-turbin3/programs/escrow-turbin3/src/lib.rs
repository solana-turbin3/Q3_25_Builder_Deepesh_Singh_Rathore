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

    pub fn make(ctx: Context<Make>,seeds: u64, amount: u64, bumps : &MakeBumps) -> Result<()> {
        ctx.accounts.make(seeds,amount,bumps)
    }
}
