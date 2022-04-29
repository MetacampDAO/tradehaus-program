use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct DistributeRewards<'info>{
  // define accounts taken in by the distribute_rewards instruction
}

impl <'info> DistributeRewards <'info> {
  // implement required functions for DistributeRewards struct
}

pub fn handler(ctx: Context<DistributeRewards>, amount: u32) -> Result<()> {
  // core instruction to allow users to distribute rewards evenly
  // to top N users as per indicated in game account. current > end_datetime
}