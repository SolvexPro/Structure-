use anchor_lang::prelude::*;

#[account]
pub struct ProtocolState {
    pub authority: Pubkey,
    pub module_count: u64,
    pub paused: bool,
    pub bump: u8,
}
