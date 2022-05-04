use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct SellItem<'info>{
  // define accounts taken in by the sell_item instruction
}

impl <'info> SellItem <'info> {
  // implement required functions for SellItems struct
}

pub fn handler(ctx: Context<SellItem>, amount: u32) -> Result<()> {
  // core instruction to allow user to choose item (coins) they wanna sell
  // indicate amount to sell, and eventually converted to fiat. end_datetime > current
}