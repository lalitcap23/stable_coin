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

pub fn intialize_config(ctx: Context<InitializeConfig>) -> Result<()>{
    process_initialize_config(ctx)


}
pub fn update_config(ctx: Context<UpdateConfig>,min_health_factor: u64) -> Result<()>{
process_update_config(ctx,min_health_factor)
}

}
