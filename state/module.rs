use anchor_lang::prelude::*;

#[account]
pub struct ModuleAccount {
    pub module_id: [u8; 32],
    pub program_id: Pubkey,
    pub enabled: bool,
    pub bump: u8,
}
