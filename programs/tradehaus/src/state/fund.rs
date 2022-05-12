use anchor_lang::prelude::*;

#[account]
pub struct Fund {
  //player pubkey
  pub player: Pubkey, 
  //BTC qty 
  pub btc_qty: u128,
  //ETH qty 
  pub eth_qty: u128,
  //LINK qty 
  pub link_qty: u128,
  //SOL qty
  pub sol_qty: u128,
  //FakeUSD qty 
  pub usd_qty: u128,
  //game pubkey,
  pub game_config: Pubkey,
}

impl Fund{
  // leave empty
}