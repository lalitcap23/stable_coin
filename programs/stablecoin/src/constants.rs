use anchor_lang::prelude::*;

#[constant]
pub const SEED_CONFIG_ACCOUNT: &[u8] = b"config_account";

pub const SEED_MINT_ACCOUNT: &[u8] = b"mint_account";

pub const MINT_DECIMALS: u8 = 9;


pub const LIQUIDITY_BONUS: u64 = 50; // 0.5% bonus

pub const LIQUIDITY_THRESHOLD: u64 = 20; // 50% threshold

pub const MIN_HEALTH_FACTOR: u64 = 150; // 150%