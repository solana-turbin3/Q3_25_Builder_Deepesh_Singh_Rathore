pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;


use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;


declare_id!("B6RsCrsXxHLgCqnXC9VPh1XqLaGpP44FQA5cVsbUe4A8");

#[program]
pub mod trustplay_v2 {
    use super::*;

    pub fn initialize(ctx: Context<InitializeRoom>,tournament_name : String) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps,tournament_name)
        
    }

    pub fn join_room(ctx: Context<JoinRoom>)-> Result<()>{
        ctx.accounts.join_room()
    }

    pub fn deposit_to_vault(ctx:Context<Deposit>,amount : u64)-> Result<()>{
        ctx.accounts.deposit_to_vault(amount)
    }

    pub fn distribute_prizes(ctx: Context<DistributePrizes>) -> Result<()> {
        ctx.accounts.distribute_prizes()
    }
}
