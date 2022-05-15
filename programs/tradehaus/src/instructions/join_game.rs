use anchor_lang::prelude::*;

use crate::state::*;
// use anchor_spl::token::{self, Mint, SetAuthority, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct JoinGame<'info> {
    //define accounts taken in by the JoinGame instructions
    #[account(
        mut, 
        constraint = Clock::get().unwrap().unix_timestamp > game_config.join_time.try_into().unwrap(), 
        constraint = Clock::get().unwrap().unix_timestamp < game_config.start_time.try_into().unwrap(), 
        constraint = game_config.current_cap < game_config.max_cap,
    )]
    pub game_config: Account<'info, Game>,

    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        init, 
        seeds = [player.to_account_info().key.as_ref(), 
                 game_config.to_account_info().key.as_ref()],
        bump,
        payer = player,
        space = 8 + 8 + (32 * 2) + (16 * 5),
    )]
    pub fund: Account<'info, Fund>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> JoinGame<'info> {
    //implement required functions for JoinGame struct
    fn set_fund_config(
      &mut self,
      fund_bump: u8,
    ) {
      self.fund.player = *self
        .player
        .to_account_info()
        .key;
      self.fund.game_config = *self
        .game_config
        .to_account_info()
        .key;
      self.fund.btc_qty = 0;
      self.fund.eth_qty = 0;
      self.fund.link_qty = 0;
      self.fund.sol_qty = 0;
      self.fund.usd_qty = self.game_config.start_usd;
      self.fund.fund_bump = fund_bump;
    }  
}

pub fn handler(
  ctx: Context<JoinGame>,
  fund_bump: u8,
) -> Result<()> {
  ctx.accounts.set_fund_config( 
    fund_bump,
  );
  Ok(())
}
