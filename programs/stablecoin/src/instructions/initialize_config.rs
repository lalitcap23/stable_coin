use anchor_lang::prelude::*;
use crate::constants::*;
use anchor_spl::token_interface::{Mint,Token2022};

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,       payer = authority,
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
 
impl<'info> InitializeConfig<'info> {
    pub fn process(&mut self,liquidity_threshold:u64,liquidity_bonus:u64,min_heath_factor:u64)->Result<()>{
        let config_account = &mut self.config_account;
        config_account.authority = self.authority.key();
        config_account.mint_account = self.mint_account.key();
        config_account.liquidity_threshold = liquidity_threshold;
        config_account.liquidity_bonus = liquidity_bonus;
        config_account.min_heath_factor = min_heath_factor;

        Ok(())
    }
}
                     