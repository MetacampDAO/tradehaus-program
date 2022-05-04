use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct BuyItem<'info>{
  // define accounts taken in by the buy_item instruction

  //not sure if this is right.. 
  #[account(mut)]
  pub player_fund: Account<'info, Fund>,

  #[account(mut)]
  pub player_token_wallet: Account<'info,TokenAccount>,

  #[account(mut)]
  pub reward_escrow: Account<'info, TokenAccount>,

  #[account(mut)]
  pub player: Signer<'info>,

  #[account(mut)]
  pub asset_mint: Account<'info, Mint>,

}

#[error_code]
pub enum ErrorCode {
  #[msg("Game is currently closed")]
  GameClosed,
  #[msg("Insufficient Balance")
  InsufficientBalance,
}

impl <'info> BuyItem <'info> {
  // implement required functions for BuyItem struct

  //check if player has enough funds to place order 
  fn has_enough_funds(&self, amount:u32) -> bool {
    let sender_tokens = &self.player_token_wallet;
    let sender_token_balance = sender_tokens.amount();
    
    if sender_token_balance > amount; {
      return True;
    } else {
      return False;
    }
  }

  //check which asset player wants to place buy order, update that asset_qty in fund account by amount in usd/asset_price 
  // will there be a vector of some sort where we have the public key of the assets? like btc, eth, sol, link etc.. 
  
  //update user fund account 
  ctx.accounts.player_fund.'' qty = ctx
  .accounts
  .player_fund
  .asset_qty
  .checked_add(amount)
  .unwrap();


  //Transfer amount of usd from player to game escrow wallet
  fn submit_buy_order(&self, amount:u32, asset: Pubkey) -> bool {
    let sender = &self.player;
    let sender_tokens = &self.player_token_wallet;
    let recipient_tokens = &self.game_escrow_wallet;
    let token_program = &self.token_program;

    let context::Transfer {
        from: sender_tokens.to_account_info(),
        to: recipient_tokens.to_account_info(),
        authority: sender.to_account_info(),
    };

    token::Transfer(
      CpiContext::new(
        token_program.to_account_info(),
        context
      ),
      amount,
    );

    return True;
  }
}

pub fn handler(ctx: Context<BuyItem>, amount: u32) -> Result<()> {
  // core instruction to allow user to choose item (coins) they wanna buy
  // indicate amount to buy, and eventually converted to fiat. end_datetime > current
  if end_datetime < current {
    return Err(error!(ErrorCode::GameClosed));

  } else if !ctx.accounts.has_enough_funds(amount) {
    return Err(error!(ErrorCode::InsufficientBalance));

  } else if end_datetime > current && ctx.accounts.has_enough_funds(amount) {
     
  }

  ctx.accounts.submit_buy_order(amount);

  Ok(())
}