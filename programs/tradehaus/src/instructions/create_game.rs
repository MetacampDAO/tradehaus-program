use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct CreateGame<'info>{
  // define accounts taken in by the CreateGame instruction
  #[account(mut, has_one = host)]
  pub game_config : Account<'info, Game>,

  #[account(mut, signer)]
  pub host : AccountInfo<'info>,

  #[account(mut)]
  pub reward_mint: Account<'info, Mint>,

  #[account(init, payer=host, space=9000)]
  ///CHECK: this is not dangerous because we do not read or write from this account 
  asset_record: Account<'info>,

  #[account(init, payer=host, space=9000)]
  pub reward_escrow: Account<'info, TokenAccount>,

  pub system_program: Program<'info, System>, 
  pub token_program: Program<'info,Token>,
}

impl <'info> CreateGame <'info> {
  // implement required functions for CreateGame struct

  //not quite sure on this portion on how to set it up.. any pointers will be great! 
  fn set_game_config(game_config, join, start, end, rewards, winners, max_players) -> Result<()> {
    let game_config.join_time = join;
    let game_config.start_time = start; 
    let game_config.end_time = end;
    let game_config.winners = winners;
    let game_config.max_cap = max_players;
    let game_config.
  }

}

pub fn handler(ctx: Context<CreateGame>, amount: u32) -> Result<()> {
  // core instruction to allow hosts to create a game account
  // must pass in required settings (join, start, end, rewards, etc) to game account
}