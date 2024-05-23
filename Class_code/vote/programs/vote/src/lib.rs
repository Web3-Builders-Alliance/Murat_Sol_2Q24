use anchor_lang::prelude::*;

declare_id!("4kbqKAWg1wg4t7UynAVnCEmiC2eonUxZm8c61xMPR77D");

#[program]
pub mod vote {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
