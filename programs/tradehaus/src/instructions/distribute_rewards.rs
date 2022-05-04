use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct DistributeRewards<'info>{
  // define accounts taken in by the distribute_rewards instruction

  //to check which player fund account is in top 10 USD balance amount 
  #[account(mut)]
  pub player_fund: Account<'info,Fund>,
  //send the rewards to associated player token wallet from top 10 fund accounts 
  #[account(mut)]
  pub player_token_wallet: Account<'info, TokenAccount>,
  //not sure how to set the reward escrow owner to game.. but yeah, transfer reward from escrow account to player token wallet 
  #[account(mut, has_one = game)]
  pub reward_escrow: Account<'info, TokenAccount>,


}

impl <'info> DistributeRewards <'info> {
  // implement required functions for DistributeRewards struct
  
}

pub fn handler(ctx: Context<DistributeRewards>, amount: u32) -> Result<()> {
  // core instruction to allow users to distribute rewards evenly
  // to top N users as per indicated in game account. current > end_datetime
}