use anchor_lang::prelude::*;

use crate::state::*;
// use anchor_spl::token::{self, Mint, SetAuthority, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct JoinGame<'info> {
    //define accounts taken in by the JoinGame instructions
    // TO-CHANGE AND REMOVE AFTER:
    // Clock::get().unwrap().unix_timestamp > game_config.join_time.try_into().unwrap() should be >= 
    #[account(
        mut, 
        constraint = Clock::get().unwrap().unix_timestamp >= game_config.join_time.try_into().unwrap(), 
        constraint = Clock::get().unwrap().unix_timestamp < game_config.start_time.try_into().unwrap(), 
        constraint = game_config.current_cap < game_config.max_cap,
    )]
    pub game_config: Account<'info, Game>,

    #[account(mut)]
    pub player: Signer<'info>,

    // TO-CHANGE AND REMOVE AFTER:
    // Add in "player-fund" as seed too
    #[account(
        init, 
        seeds = [b"player-fund".as_ref(), 
                 player.to_account_info().key.as_ref(),
                 game_config.to_account_info().key.as_ref()],
        bump,
        payer = player,
        space = 8 + 8 + (32 * 2) + (16 * 5),
    )]
    pub player_fund: Account<'info, Fund>,

    pub system_program: Program<'info, System>
}

impl<'info> JoinGame<'info> {
    //implement required functions for JoinGame struct

    // TO-CHANGE AND REMOVE AFTER:
    // Change name to set_fund_incr_cap_config
    fn set_fund_incr_cap_config(
      &mut self,
      player_fund_bump: u8,
    ) {
      self.player_fund.player = *self
        .player
        .to_account_info()
        .key;
      self.player_fund.game_config = *self
        .game_config
        .to_account_info()
        .key;
      self.player_fund.btc_qty = 0;
      self.player_fund.eth_qty = 0;
      self.player_fund.link_qty = 0;
      self.player_fund.sol_qty = 0;
      self.player_fund.usd_qty = self.game_config.start_usd;
      self.player_fund.fund_bump = player_fund_bump;

      // TO-CHANGE AND REMOVE AFTER:
      // increment game_config.current_cap below
      self.game_config.current_cap += 1;
    }  
}

pub fn handler(
  ctx: Context<JoinGame>,
  player_fund_bump: u8,
) -> Result<()> {
  ctx.accounts.set_fund_incr_cap_config( 
    player_fund_bump,
  );
  Ok(())
}
