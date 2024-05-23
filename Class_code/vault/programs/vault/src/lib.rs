use anchor_lang::prelude::*;

declare_id!("DGzACNZ7YcnLEA8B7ZtJPt25bxXi95KXxrQXCLwvXARn");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
