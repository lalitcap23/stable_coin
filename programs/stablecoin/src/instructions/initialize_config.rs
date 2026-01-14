use anchor_lang::prelude::*;
use crate::constants::*;
use anchor_spl::token_interface::{Mint,Token2022};
use crate::state::config::Config;
#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,   
        payer = authority,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump,
        space = 8 + Config::INIT_SPACE, // Add space as needed
    )]
    pub config_account:Account<'info,Config>,// it is the first config account to inititialize the program


    #[account(
        init,
        payer = authority,
        seeds = [SEED_MINT_ACCOUNT],
        bump,  
        mint::decimals = MINT_DECIMALS,
        mint::authority = mint_account,
        mint::freeze_authority = mint_account,
        mint::token_program = token_program,

    )]
    pub mint_account: Account<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}
 
pub fn process_initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
    *ctx.accounts.config_account = Config{
     authority: ctx.accounts.authority.key(),
      mint_account: ctx.accounts.mint_account.key(),
      liquidity_threshold: LIQUIDITY_THRESHOLD,
        liquidity_bonus: LIQUIDITY_BONUS,
        min_heath_factor: MIN_HEALTH_FACTOR,
        bump: ctx.bumps.config_account,
        bump_mint_acount: ctx.bumps.mint_account,
    };
    Ok(())
}
