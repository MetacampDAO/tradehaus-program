use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct SwapItems<'info>{
  // define accounts taken in by the swap_items instruction

  //not sure if this is right.. 
  #[account(mut)]
  pub player_fund: Account<'info, Fund>,

  // #[account(mut)]
  // pub player_token_wallet: Account<'info,TokenAccount>,

  // #[account(mut)]
  // pub reward_escrow: Account<'info, TokenAccount>,

  // #[account(mut)]
  // pub player: Signer<'info>,

  // #[account(mut)]
  // pub asset_mint: Account<'info, Mint>,

  // #[account(mut)]
  // pub game_config: Account<'info, Game>,

}

// #[error_code]
// pub enum ErrorCode {
//   #[msg("Game is currently closed!")]
//   GameClosed,
//   #[msg("Host not allowed!")
//   HostDenied,
//   #[msg("Max player capacity reached!")]
//   MaxPlayers,
// }

impl <'info> SwapItems <'info> {
  // // implement required functions for BuyItem struct

  // //check if player is new.. if new add into list of players  ( not sure how to implement boolean condition for this)
  // fn check_player_is_new (&self, player: Pubkey) -> bool {
  //   if player == game_config.host.key() {
  //     return Err(Error!(ErrorCode::HostDenied));
  //   }
  //   let player_list = &self.game_config.players;
  //   if player_list.iter().any(|x| *x == player) {
  //     return True;
  //   }
  //   let default_pubkey = Pubkey::default();
  //   if let Some(idx) = player_list.iter().position(|x| *x == default_pubkey) {
  //     player_list[idx] = player;
  //   } else {
  //       return Err(Error!(ErrorCode::MaxPlayers));
  //   }
  //   Ok (())
  // }

  // //check if player has enough funds to place order 
  // fn has_enough_funds(&self, amount:u32) -> bool {
  //   let sender_tokens = &self.player_token_wallet;
  //   let sender_token_balance = sender_tokens.amount();
    
  //   if sender_token_balance > amount; {
  //     return True;
  //   } else {
  //     return False;
  //   }
  // }
  // //check which asset player wants to place buy order, update that asset_qty in fund account by amount in usd/asset_price 
  // // will there be a vector of some sort where we have the public key of the assets? like btc, eth, sol, link etc.. 
  // //update user fund account
  // fn place_order(&self, amount:u32) -> Result<()> {
  //   let qty = amount / asset_price
  //   ctx.accounts.player_fund.'' qty = ctx
  //   .accounts
  //   .player_fund
  //   .asset_qty
  //   .checked_add(amount)
  //   .unwrap();
  // }
}

pub fn handler(ctx: Context<SwapItems>, amount: u32) -> Result<()> {
  // // core instruction to allow user to choose item (coins) they wanna buy
  // // indicate amount to buy, and eventually converted to fiat. end_datetime > current
  // //need price feed... everytime player places order, grab associated price feed address 
  // if end_datetime < current {
  //   return Err(error!(ErrorCode::GameClosed));

  // } else if !ctx.accounts.has_enough_funds(amount) {
  //   return Err(error!(ErrorCode::InsufficientBalance));

  // } else if end_datetime > current && ctx.accounts.has_enough_funds(amount) {
     
  // }

  // ctx.accounts.place_order(amount);

  Ok(())
}