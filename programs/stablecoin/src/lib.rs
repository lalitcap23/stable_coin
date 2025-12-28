pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2Wg97zfSSFgdcSyHcefD9dgafn5iXwjyQzpfzuX8tqjm");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize(ctx: Context<InitializeConfig>) -> Result<()> {
        Ok(())
    }
}
