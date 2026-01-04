use anchor_lang::prelude::*;
use crate::constants::*;

#[derive(Accounts)]
pub struct UpdateConfig<'info>{
    #[account{
        mut,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
        has_one = authority,
    }]
    pub config_account: Account<'info,Config>,
}

 pub fn process_initialize_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()>{
 Ok(())
 }


