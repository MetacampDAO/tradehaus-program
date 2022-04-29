use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct BuyItem<'info>{
  // define accounts taken in by the buy_item instruction
}

impl <'info> BuyItem <'info> {
  // implement required functions for BuyItem struct
}

pub fn handler(ctx: Context<BuyItem>, amount: u32) -> Result<()> {
  // core instruction to allow user to choose item (coins) they wanna buy
  // indicate amount to buy, and eventually converted to fiat. end_datetime > current
}