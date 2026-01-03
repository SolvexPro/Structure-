use anchor_lang::prelude::*;

pub mod constants;
pub mod state;
pub mod instructions;
pub mod events;
pub mod errors;

use instructions::*;

declare_id!("SoLvEX111111111111111111111111111111111");

#[program]
pub mod solvex {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    pub fn register_module(
        ctx: Context<RegisterModule>,
        module_id: [u8; 32],
    ) -> Result<()> {
        instructions::register_module::handler(ctx, module_id)
    }

    pub fn execute(
        ctx: Context<Execute>,
        module_id: [u8; 32],
        payload: Vec<u8>,
    ) -> Result<()> {
        instructions::execute::handler(ctx, module_id, payload)
    }

    pub fn pause(ctx: Context<Pause>, value: bool) -> Result<()> {
        instructions::pause::handler(ctx, value)
    }
}
