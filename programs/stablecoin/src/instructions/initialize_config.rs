use anchor_lang::prelude::*;
use crate::{constants::*, state::Config};

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
    pub config_account: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeConfig<'info> {
    pub fn initialize(
        &mut self,
        liquidity_threshold: u64,
        liquidity_bonus: u64,
        min_heath_factor: u64,
        bumps: &InitializeConfigBumps,
    ) -> Result<()> {
        self.config_account.set_inner(Config {
            authority: self.authority.key(),
            mint_account: Pubkey::default(), // Set when mint is created
            liquidity_threshold,
            liquidity_bonus,
            min_heath_factor,
            bump: bumps.config_account,
            bump_mint_acount: 0, // Set when mint is created
        });
        
        Ok(())
    }
}
  