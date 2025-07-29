use anchor_lang::prelude::*;

declare_id!("7x4zQWLkLwMygSNYP3FwK172QpHTGirudn8TD5d8a3oY");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
