use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct DistributeRewards<'info>{
  // define accounts taken in by the distribute_rewards instruction

  //to check which player fund account is in top 10 USD balance amount 
  #[account(mut)]
  pub player_fund: Account<'info,Fund>,

  // //send the rewards to associated player token wallet from top 10 fund accounts 
  // #[account(mut)]
  // pub player_token_wallet: Account<'info, TokenAccount>,

  // //not sure how to set the reward escrow owner to game.. but yeah, transfer reward from escrow account to player token wallet 
  // #[account(mut, has_one = game)]
  // pub reward_escrow: Account<'info, TokenAccount>,

  // //game account which should store all the players account details for checking top N balances and then distributing rewards
  // pub game_config: Account<'info, Game>, 
}

// #[error_code]
// pub enum ErrorCode {
//   #[msg("Game is currently opened")]
//   GameOpened,
// }

impl <'info> DistributeRewards <'info> {
  // // implement required functions for DistributeRewards struct
  // //grab addresses from vecotr / hashmap from game account 
   
  // fn transfer_reward(&self, amount:u32) -> Result<()> {
    
  //   let sender = &self.game;
  //   let sender_tokens = &self.reward_escrow;
  //   let recipient_tokens = &self.player_token_wallet;
  //   let token_program = &self.token_program;

  //   let context::Transfer {
  //       from: sender_tokens.to_account_info(),
  //       to: recipient_tokens.to_account_info(),
  //       authority: sender.to_account_info(),
  //   };
  //   seeds

  //   token::Transfer(
  //     CpiContext::new_with_signer(
  //       token_program.to_account_info(),
  //       context
  //     ),
  //     amount,
  //   );


  
}

pub fn handler(ctx: Context<DistributeRewards>, amount: u32) -> Result<()> {
  // // core instruction to allow users to distribute rewards evenly
  // // to top N users as per indicated in game account. current > end_datetime
  // if current < end_datetime {
  //   return Err(error!(ErrorCode::GameOpened));

  Ok(())
}