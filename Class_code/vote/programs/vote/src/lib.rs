use anchor_lang::prelude::*;

declare_id!("4kbqKAWg1wg4t7UynAVnCEmiC2eonUxZm8c61xMPR77D");

#[program]
pub mod vote {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _url: String) -> Result<()> {
        ctx.accounts.initialize();
    }

    pub fn upvote(ctx: Context<Vote>, _url: String) -> Result<()> {
        Ok(())
    }

    pub fn downvote(ctx: Context<Vote>, _url: String) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
