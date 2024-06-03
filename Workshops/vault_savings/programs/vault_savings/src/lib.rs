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

// -----------------------------

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"state".as_ref(), user.key().as_ref()],
        bump,
        space = VaultState::INIT_SPACE
    )]
    pub state: Account<'info, VaultState>,
    #[account(
        seeds = [b"vault".as_ref(), state.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, amount: u64, bumps: &InitializeBumps) -> Result<()> {
        self.state.amount = amount;
        self.state.vault_bump = bumps.vault;
        self.state.state_bump = bumps.state;

        Ok(())
    }
}



// -----------------------------

#[account]
pub struct VaultState {
    pub amount: u64,
    pub vault_bump: u8,
    pub state_bump: u8
}

impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 8 + 1 + 1;
}