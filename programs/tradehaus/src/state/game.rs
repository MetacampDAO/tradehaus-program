use anchor_lang::prelude::*;

#[account]
pub struct Game {
  //game host pubkey 
  pub host: Pubkey,
  //host reward token account 
  pub host_reward_account: Pubkey,
  //amount of reward to be set by host 
  pub reward_amount: u128,
  //join time 
  pub join_time: u64,
  //start time
  pub start_time: u64,
  //end time 
  pub end_time: u64,
  //max no. of participants 
  pub max_cap: u64,
  //no. of winnners 
  pub winners: u8,
  //reward token mint 
  pub reward_mint: Pubkey,
  //game escrow pubkey
  pub escrow: Pubkey,
  //game stated 
  pub game_ended: bool,
}

impl Game{
  // leave empty
}