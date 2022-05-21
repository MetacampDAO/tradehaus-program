use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct SwapItems<'info> {
  // define accounts taken in by the swap_items instruction
  #[account(
      mut,
      constraint = player_fund.player == *player.key,
      constraint = player_fund.game_config == game_config.key(),
  )]
  pub player_fund: Account<'info, Fund>,

  #[account(mut)]
  pub player: Signer<'info>,

  #[account(
    mut,
    constraint = Clock::get().unwrap().unix_timestamp >= game_config.start_time.try_into().unwrap(),
    constraint = Clock::get().unwrap().unix_timestamp < game_config.end_time.try_into().unwrap(),
  )]
  pub game_config: Account<'info, Game>,
}

impl<'info> SwapItems<'info> {
  fn swap_items(&mut self, amount: u128, sell_coin: u8, buy_coin: u8) {
    let mut sell_coin_fund = match sell_coin {
      1 => self.player_fund.btc_qty,
      2 => self.player_fund.eth_qty,
      3 => self.player_fund.link_qty,
      4 => self.player_fund.sol_qty,
      5 => self.player_fund.usd_qty,
      _ => self.player_fund.usd_qty,
    };
    if sell_coin_fund < amount {
      // throw error
      panic!("Error");
    }
    sell_coin_fund -= amount;

    let mut buy_coin_fund = match buy_coin {
      1 => self.player_fund.btc_qty,
      2 => self.player_fund.eth_qty,
      3 => self.player_fund.link_qty,
      4 => self.player_fund.sol_qty,
      5 => self.player_fund.usd_qty,
      _ => self.player_fund.usd_qty,
    };
    buy_coin_fund += convert_size(1) // price of sell_coin set to 1 for now
      .checked_mul(amount)
      .unwrap()
      .checked_div(convert_size(1)) //price of buy_coin set to 1 for now
      .unwrap();
  }
}

fn convert_size(v: u8) -> u128 {
  u128::from(v)
}

pub fn handler(ctx: Context<SwapItems>, amount: u128, sell_coin: u8, buy_coin: u8) -> Result<()> {
  // check if amount is <= coin qty... not sure how to write this
  // check if all coin_qty is 0
  //if yes, then check that amount <= player_fund.usd_qty
  // if no , then check if amount <= player_fund.sell_coin_qty
  // core instruction to allow user to choose item (coins) they wanna buy
  ctx.accounts.swap_items(amount, sell_coin, buy_coin);

  // } else if !ctx.accounts.has_enough_funds(amount)
  Ok(())
}
