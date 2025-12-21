use anchor_lang::{prelude::*, solana_program::{config, stake::config::Config}};
use crate::constants::*;

#[derive(Accounts)]
pub struct InitializeConfig {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump,
        space = 8 + Config::INIT_SPACE, // Add space as needed
    )]
    pub config_account:Account<'info,Config>,
}

pub imp<'info> InitializeConfig<'info> {
    Ok(())
}
  