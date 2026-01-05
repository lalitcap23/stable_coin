use anchor_lang::prelude::*;
use crate::constants::*;
use anchor_spl::token_interface::Mint;
use crate::state::config::Config;
use crate::state::collateral::Collateral;

#[derive(Accounts)]
pub struct DepositColMintToken<'info>{
    #[account(mut)]
    pub depositor: Signer<'info>,
    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
        has_one = mint_account,
    )]
    pub config_account: Account<'info,Config>,
    #[account(mut)]
    pub mint_account: Account<'info, Mint>,

    #[account(
     init_if_needed,
     payer = depositor,
     space = 8 + 32 + 8 + 1, // Adjust space as needed
     seeds = [SEED_COLLATERAL_ACCOUNT, depositor.key().as_ref()],
     bump,

    )]
    pub collateral_account: Account<'info, Collateral>,
    pub system_program: Program<'info, System>,

    
    
 }
   