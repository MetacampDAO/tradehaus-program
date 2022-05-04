use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct CreateGame<'info>{
  // define accounts taken in by the CreateGame instruction
  #[account(init, payer=host, space=9000)]
  ///CHECK: this is not dangerous because we do not read or write from this account 
  asset_record: Account<'info>,

  #[account(init, payer=host, space=9000)]
  ///CHECK: this is not dangerous because we do not read or write from this account
  asset_price_record: UncheckedAccount<'info>,

  #[account(mut)]
  pub host: Signer<'info>,

  #[account(mut)]
  pub reward_mint: Account<'info, Mint>,

  #[account(init, payer=host, space=9000)]
  pub reward_escrow: Account<'info, TokenAccount>,

  pub system_program: Program<'info, System>, 
  pub token_program: Program<'info,Token>,
}

impl <'info> CreateGame <'info> {
  // implement required functions for CreateGame struct

  //not quite sure on this portion on how to set it up.. any pointers will be great! 

}

pub fn handler(ctx: Context<CreateGame>, amount: u32) -> Result<()> {
  // core instruction to allow hosts to create a game account
  // must pass in required settings (join, start, end, rewards, etc) to game account
}