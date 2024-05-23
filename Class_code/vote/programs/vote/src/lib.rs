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
#[instruction(_url: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init,
        payer = payer,
        seeds = [_url.as_bytes().as_ref()],
        bump,
        space = VoteState::INIT_SPACE
    )]
    pub vote_account: Account<'info, VoteState>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.vote_account.score = 0;
        self.vote_account.bump = bumps.vote_account;

        Ok(())
    }
}


#[account]
pub struct VoteState {
    pub score: i64,
    pub bump: u8,
}

impl Space for VoteState {
    const INIT_SPACE: usize = 8 + 8 + 1;
}