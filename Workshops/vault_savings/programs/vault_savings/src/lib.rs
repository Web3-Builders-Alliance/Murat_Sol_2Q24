use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

declare_id!("BFSM8WN4jSBAtytutZZhm14AsVQiazW5ncfFQtttBFt7");

#[program]
pub mod vault_savings {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        ctx.accounts.initialize(amount, &ctx.bumps)?;

        Ok(())
    }

    pub fn deposit(ctx: Context<Operations>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;

        Ok(())
    }

    pub fn withdrawal(ctx: Context<Operations>, amount: u64) -> Result<()> {
        ctx.accounts.withdrawal(amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
