use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct CreateGame<'info>{
  // define accounts taken in by the CreateGame instruction
  #[account(mut, init, payer = host, space=9000)]
  pub game_config : Account<'info, Game>,

  #[account(mut)]
  pub host : Signer<'info>,

  #[account(mut, constraint = host_reward_account.mint == reward_mint.key,)]
  pub host_reward_account: Account<'info, TokenAccount>,

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
  fn set_game_config(game_config, join, start, end, winners, max_players, reward_mint, game_ended, reward_amount) -> Result<()> {
    game_config.join_time = join;
    game_config.start_time = start; 
    game_config.end_time = end;
    game_config.winners = winners;
    game_config.max_cap = max_players;
    game_config.reward_mint = reward_mint;
    game_config.game_ended = game_ended;
    game_config.reward_amount = reward_amount;

    Ok(())
  }

  fn transfer_host_reward(&self, amount: u64) -> Result<()> {
    let sender = &self.host;
    let sender_of_tokens = &self.host_reward_account;
    let recipient_of_tokens = &self.reward_escrow;
    let token_program = &self.token_program;

    let context = Transfer {
      from: sender_of_tokens.to_account_info(),
      to: recipient_of_tokens.to_account_info(),
      authority: sender.to_account_info(),
    };

    token::transfer (
      CpiContext::new(
        token_program.to_account_info(),
        context,
      ),
      amount,
    );
    Ok(())
  }

}

pub fn handler(ctx: Context<CreateGame>, join: u64, start: u64, end: u64, winners: u64, max_players: u64, reward_mint: Pubkey, game_ended: bool, reward_amount: u128) -> Result<()> {
  // core instruction to allow hosts to create a game account
  // must pass in required settings (join, start, end, rewards, etc) to game account
}