use anchor_lang::prelude::*;


mod constants;
mod state;
mod contexts;
use contexts::*;
mod error;
mod helpers;

declare_id!("Z82xSvKryNqCEF1VwoL6myvNCF2JgbRjsW1F9TJ8y9q");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64, fee: u16, authority: Option<Pubkey>) -> Result<()> {
        ctx.accounts.init(&ctx.bumps, seed, fee, authority);

        Ok(())
    }

    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64,
        max_x: u64,
        max_y: u64,
        expiration: i64,
    ) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y, expiration);

        Ok(())
    }

    pub fn swap(
        ctx: Context<Swap>,
        is_x: bool,
        amount: u64,
        min: u64,
        expiration: i64
    ) -> Result<()> {
        ctx.accounts.swap(is_x,amount,min,expiration);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
