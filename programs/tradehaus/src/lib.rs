use anchor_lang::prelude::*;

use instructions::*;

pub mod state;
pub mod instructions;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod tradehaus {
    use super::*;

    pub fn create_game(ctx: Context<CreateGame>,
        join: u64, 
        start: u64, 
        end: u64, 
        start_usd: u64, 
        winners: u8, 
        max_players: u64, 
        reward_amount: u64,
        reward_escrow_bump: u8) -> Result<()> {
        instructions::create_game::handler(
            ctx,
            join, 
            start, 
            end, 
            start_usd, 
            winners, 
            max_players,
            reward_amount,
            reward_escrow_bump)
    }

    pub fn join_game(ctx: Context<JoinGame>, fund_bump: u8) -> Result<()> {
        instructions::join_game::handler(ctx, fund_bump)
    }

    pub fn swap_items(ctx: Context<SwapItems>, amount: u32) -> Result<()> {
        instructions::swap_items::handler(ctx, amount)
    }

    pub fn distribute_rewards(ctx: Context<DistributeRewards>, amount: u32) -> Result<()> {
        instructions::distribute_rewards::handler(ctx, amount)
    }
}
