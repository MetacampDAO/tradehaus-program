use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct CreateGame<'info>{
  // define accounts taken in by the CreateGame instruction
}

impl <'info> CreateGame <'info> {
  // implement required functions for CreateGame struct
}

pub fn handler(ctx: Context<CreateGame>, amount: u32) -> Result<()> {
  // core instruction to allow hosts to create a game account
  // must pass in required settings (join, start, end, rewards, etc) to game account
}