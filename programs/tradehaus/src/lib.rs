use anchor_lang::prelude::*;

use instructions::*;

pub mod state;
pub mod instructions;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod tradehaus {
    use super::*;

    pub fn create_game(ctx: Context<CreateGame>) -> Result<()> {
        instructions::create_game::handler(ctx)
    }

    pub fn buy_item(ctx: Context<BuyItem>, amount: u32) -> Result<()> {
        instructions::buy_item::handler(ctx, amount)
    }

    pub fn sell_item(ctx: Context<SellItem>, amount: u32) -> Result<()> {
        instructions::sell_item::handler(ctx, amount)
    }

    pub fn distribute_rewards(ctx: Context<DistributeRewards>) -> Result<()> {
        instructions::distribute_rewards::handler(ctx)
    }
}
