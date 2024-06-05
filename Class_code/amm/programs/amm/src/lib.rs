use anchor_lang::prelude::*;

declare_id!("Z82xSvKryNqCEF1VwoL6myvNCF2JgbRjsW1F9TJ8y9q");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
