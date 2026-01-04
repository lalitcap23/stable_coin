use anchor_lang::prelude::*;
use crate::constants::*;

#[derive(accounts)]
pub struct DepositColMintToken<'info>{
    #[account(mut,
    seeds = [SEED_CONFIG_ACCOUNT],
    bump = config_account.bump,
    has_one = authority,
    )]
    pub config_account: Account<'info,Config>,

}

 pub fn process_deposit_col_mint_token(ctx: Context<DepositColMintToken>) -> Result<()>{

 let config_account = &mut ctx.accounts.config_account;
 
  config_account.collected_mint_tokens +=1;
 Ok(())
 }