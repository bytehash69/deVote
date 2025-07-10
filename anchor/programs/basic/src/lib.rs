#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

declare_id!("JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H");

#[program]
pub mod basic {
    use super::*;

    pub fn initialize_poll(
        ctx: Context<InitializePoll>,
        poll_id: u64,
        question: String,
        start_time: u64,
        end_time: u64,
    ) -> Result<()> {
        let poll_account = &mut ctx.accounts.poll;

        poll_account.poll_id = poll_id;
        poll_account.question = question;
        poll_account.start_time = start_time;
        poll_account.end_time = end_time;
        poll_account.creator = *ctx.accounts.signer.key;
        poll_account.total_options = 0;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Poll::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub poll_id: u64,
    pub creator: Pubkey,
    pub start_time: u64,
    pub end_time: u64,
    #[max_len(200)]
    pub question: String,
    pub total_options: u64
}

#[account]
#[derive(InitSpace)]
pub struct VoteOption {
    pub option_id: u64,
    pub poll_id: u64,
    #[max_len(60)]
    pub title: String
}

#[account]
#[derive(InitSpace)]
pub struct VoteRecord {
    pub has_voted: bool,
    pub option_id: u64,
    pub poll_id: u64
}