use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};

use anchor_spl::token_2022::{mint_to, MintTo, Token2022};
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::constants::*;


pub fn mint_token<'info>(
    mint_account: InterfaceAccount<'info, Mint>,
    token_account: InterfaceAccount<'info, TokenAccount>,
    token_program: Program<'info, Token2022>,
    bump: u8,
    amount: u64,
) -> Result<()> {

    let signer_seeds: &[&[&[u8]]] = &[&[
        SEED_MINT_ACCOUNT,
        &[bump], 
    ]];

    let cpi_accounts = MintTo {
        mint: mint_account.to_account_info(),
        to: token_account.to_account_info(),
        authority: mint_account.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );

    mint_to(cpi_ctx, amount)?;

    Ok(())
}


pub fn deposit_sol<'info>(
    from: Signer<'info>,
    to: SystemAccount<'info>,
    system_program: Program<'info, System>,
    amount: u64,
) -> Result<()> {

    let cpi_accounts = Transfer {
        from: from.to_account_info(),
        to: to.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        system_program.to_account_info(),
        cpi_accounts,
    );

    system_program::transfer(cpi_ctx, amount)?;

    Ok(())
}
