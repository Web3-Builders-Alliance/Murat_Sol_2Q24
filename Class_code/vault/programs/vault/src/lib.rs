use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

declare_id!("DGzACNZ7YcnLEA8B7ZtJPt25bxXi95KXxrQXCLwvXARn");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;

        Ok(())
    }

    pub fn close(ctx: Context<close>) -> Result<()> {
        ctx.accounts.close()?;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize {}
