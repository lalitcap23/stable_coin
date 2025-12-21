use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Config{
     pub authority: Pubkey,
    pub mint_account: Pubkey,
    pub liquidity_threshold: u64,
    pub liquidity_bonus: u64,
    pub min_heath_factor: u64,
    pub bump: u8,
    pub bump_mint_acount: u8,

}
