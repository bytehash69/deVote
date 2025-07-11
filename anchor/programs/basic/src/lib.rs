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

    pub fn add_vote_option(
        ctx: Context<AddVoteOption>,
        poll_id: u64,
        option_id: u64,
        option_title: String
    ) -> Result<()> {
        let vote_option = &mut ctx.accounts.vote_option;
        let poll = &mut ctx.accounts.poll;

        vote_option.option_id = option_id;
        vote_option.title = option_title;
        vote_option.poll_id = poll_id;

        poll.total_options += 1;
        
        Ok(())
    }

    pub fn record_vote_option(
        ctx: Context<Vote>,
        poll_id: u64,
        vote_option_id: u64
    ) -> Result<()> {
        let poll_account = &mut ctx.accounts.poll;
        let vote_record = &mut ctx.accounts.vote_record;

        vote_record.has_voted = true;
        vote_record.option_id = vote_option_id;
        vote_record.poll_id = poll_id;

        Ok(())
    }

    pub fn delete_poll(
        ctx:Context<DeletePoll>,
        poll_id: u64
    ) -> Result<()> {
        let poll_account = &mut ctx.accounts.poll_account;

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

#[derive(Accounts)]
#[instruction(poll_id: u64,vote_option_id: u64)]
pub struct AddVoteOption<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    #[account(
        init,
        payer = signer,
        space = 8 + VoteOption::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref(),vote_option_id.to_le_bytes().as_ref()],
        bump
    )]
    pub vote_option: Account<'info,VoteOption>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64,vote_option_id: u64)]
pub struct Vote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(),vote_option_id.to_le_bytes().as_ref()],
        bump
    )]
    pub vote_option: Account<'info,VoteOption>,

    #[account(
        init,
        payer = signer,
        space = 8 + VoteRecord::INIT_SPACE,
        seeds = [b"vote",poll_id.to_le_bytes().as_ref(),vote_option_id.to_le_bytes().as_ref()],
        bump
    )]
    pub vote_record: Account<'info,VoteRecord>,

    pub system_program: Program<'info,System>
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct DeletePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump,
        constraint = poll_account.creator == * signer.key,
        close = signer
    )]
    pub poll_account: Account<'info,Poll>,

    pub system_program: Program<'info,System>
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