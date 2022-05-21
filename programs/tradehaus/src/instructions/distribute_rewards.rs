use anchor_lang::prelude::*;

use crate::state::*;
use anchor_spl::token::{self, Mint, SetAuthority, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
  // define accounts taken in by the distribute_rewards instruction
  #[account(
      mut,
      constraint = player_fund.player == *player.key,
      constraint = player_fund.game_config == game_config.key(),
  )]
  pub player_fund: Account<'info, Fund>,

  #[account(mut)]
  pub player: Signer<'info>,

  #[account(mut)]
  pub player_token_wallet: Account<'info, TokenAccount>,

  #[account(mut)]
  pub host: Signer<'info>,

  #[account(mut)]
  pub reward_mint: Account<'info, Mint>,

  #[account(
      mut, 
      seeds = [b"reward-escrow".as_ref(), game_config.to_account_info().key.as_ref()],
      bump,
      token::authority = host,
      token::mint = reward_mint,
  )]
  pub reward_escrow: Account<'info, TokenAccount>,

  #[account(
      mut,
      constraint = Clock::get().unwrap().unix_timestamp < game_config.end_time.try_into().unwrap(),
  )]
  pub game_config: Account<'info, Game>,

  pub token_program: AccountInfo<'info>,
}

impl<'info> DistributeRewards<'info> {
  // // implement required functions for DistributeRewards struct
  // //grab addresses from vecotr / hashmap from game account
  fn transfer_reward(&self, amount:u64) -> Result<()> {
    let sender = &self.game_config;
    let sender_tokens = &self.reward_escrow;
    let recipient_tokens = &self.player_token_wallet;
    let token_program = &self.token_program;

    let context = Transfer {
        from: sender_tokens.to_account_info(),
        to: recipient_tokens.to_account_info(),
        authority: sender.to_account_info(),
    };

    token::transfer(
      CpiContext::new_with_signer(token_program.to_account_info(),context),
      amount,
    );
  }
}

pub fn handler(ctx: Context<DistributeRewards>, amount: u32) -> Result<()> {
  // // core instruction to allow users to distribute rewards evenly
  // // to top N users as per indicated in game account. current > end_datetime

  Ok(())
}
