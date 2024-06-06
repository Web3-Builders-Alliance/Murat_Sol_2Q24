use anchor_lang::prelude::*;

declare_id!("2ZCEiywq7CqqiNo726wDeX9croaZwBxwtExGwf4dg7s7");

mod state;
mod contexts;
mod contexts::*;

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {

        ctx.accounts.init(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
