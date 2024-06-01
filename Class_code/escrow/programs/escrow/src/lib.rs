use anchor_lang::prelude::*;

declare_id!("2ZCEiywq7CqqiNo726wDeX9croaZwBxwtExGwf4dg7s7");

mod state;
mod contexts;
// mod contexts::*;

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
