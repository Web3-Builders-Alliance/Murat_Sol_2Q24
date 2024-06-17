use anchor_lang::prelude::*;

declare_id!("F2HAqgH9yVEVx1isB2wrAhC9Ruo28w48JxsWeMAASmqU");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
