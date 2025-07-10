use anchor_lang::prelude::*;

declare_id!("JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H");

#[program]
pub mod basic {
    use super::*;
}

#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub poll_id: u64,
    pub owner: Pubkey,
    pub start_time: u64,
    pub end_time: u64,
    #[max_len(200)]
    pub question: String
}

#[account]
#[derive(InitSpace)]
pub struct Option {
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
    pub pool_id: u64
}