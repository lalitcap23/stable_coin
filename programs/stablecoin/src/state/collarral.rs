use anchor_lang::prelude::*;

#[account]
#[derive(initspace,debug)]
pub struct Collateral {
    pub depositor: Pubkey,
    pub sol_account: Pubkey,
    pub token_account: Pubkey,// ata acount for hte depo
    pub lamports_balance: u64,
    pub amount_minted: u64,
    pub is_initialized: bool,
    pub bump: u8,
    pub bump_sol_amount: u8,
}
