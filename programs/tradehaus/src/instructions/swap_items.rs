use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct SwapItems<'info> {
  // define accounts taken in by the swap_items instruction
  #[account(
      mut,
      constraint = player_fund.player == *player.to_account_info().key,
      constraint = player_fund.game_config == *game_config.to_account_info().key,
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

  pub system_program: Program<'info, System>,
}

impl<'info> SwapItems<'info> {
  fn swap_items(&mut self, amount: u128, sell_coin: u8, buy_coin: u8) {
    match sell_coin {
      1 => {
        self.player_fund.btc_qty -= amount;
        Ok(self.player_fund.btc_qty)
      },
      2 => {
        self.player_fund.eth_qty -= amount;
        Ok(self.player_fund.eth_qty)
      },
      3 => {
        self.player_fund.link_qty -= amount;
        Ok(self.player_fund.link_qty)
      },
      4 => {
        self.player_fund.sol_qty -= amount;
        Ok(self.player_fund.sol_qty)
      },
      5 => {
        self.player_fund.usd_qty -= amount;
        Ok(self.player_fund.usd_qty)
      },
      _ => Err(ProgramError::Custom(1)),
    }.unwrap();

    let (sell_coin_price, buy_coin_price) = (1 as u128, 1 as u128);

    match buy_coin {
      1 => {
        self.player_fund.btc_qty += find_buy_amt(amount, sell_coin_price, buy_coin_price);
        Ok(self.player_fund.btc_qty)
      }
      2 => {
        self.player_fund.eth_qty += find_buy_amt(amount, sell_coin_price, buy_coin_price);
        Ok(self.player_fund.eth_qty)
      }
      3 => {
        self.player_fund.link_qty += find_buy_amt(amount, sell_coin_price, buy_coin_price);
        Ok(self.player_fund.link_qty)
      }
      4 => {
        self.player_fund.sol_qty += find_buy_amt(amount, sell_coin_price, buy_coin_price);
        Ok(self.player_fund.sol_qty)
      }
      5 => {
        self.player_fund.usd_qty += find_buy_amt(amount, sell_coin_price, buy_coin_price);
        Ok(self.player_fund.usd_qty)
      },
      _ => Err(ProgramError::Custom(1)),
    }.unwrap();
  }
}

fn find_buy_amt(amount: u128, sell_coin_price: u128, buy_coin_price: u128) -> u128 {
  sell_coin_price
    .checked_mul(amount)
    .unwrap()
    .checked_div(buy_coin_price)
    .unwrap()
}

pub fn handler(ctx: Context<SwapItems>, amount: u128, sell_coin: u8, buy_coin: u8) -> Result<()> {
  // check if amount is <= coin qty... not sure how to write this
  // check if all coin_qty is 0
  //if yes, then check that amount <= player_fund.usd_qty
  // if no , then check if amount <= player_fund.sell_coin_qty
  // core instruction to allow user to choose item (coins) they wanna buy
  ctx.accounts.swap_items(amount, sell_coin, buy_coin);
  Ok(())
}
