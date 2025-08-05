pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod context;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
pub use crate::context::*;



declare_id!("GNRzq9iJBiUirwbpsa1kuE1xsLm5i8kRZqH5hMT9q4NN");

#[program]
pub mod joinroom {

    use super::*;

    pub fn initialize(ctx: Context<InitializeRoom>,name :String) -> Result<()> {
        initialize_room::handler(ctx,name)
    }

    pub fn joinroom(ctx: Context<JoinRoom>)-> Result<()>{
        join_room::handler(ctx)
    }

    pub fn initialize_vault( ctx : Context<InitializeVault>,amount : u64)-> Result<()>{
        initialize_room_vault::handler(ctx, amount)
    }

    pub fn distribute_prize<'info>( ctx : Context<'_,'_,'info,'info,DistributePrize<'info>>)-> Result<()>{
        ctx.accounts.distribute_prizes(ctx.remaining_accounts)
    }
}
