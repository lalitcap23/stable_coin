use anchor_lang::prelude::*;
use anchor_spl::{Token2022::{mint_to,MintTo,token_2022},token_interface::{Mint,TokenAccount}};

use crate::constants::*;

pub fn mint_token<'info>(
    mint_account: InterfaceAccount<'info,Mint>,
    token_account: InterfaceAccount<'info,TokenAccount>,
    token_program: Program<'info,Token2022>,
    bump: u8,
    amount: u64,
    )->Result<()>{
      let signer_seeds: &[&[&[u8]]] = &[&[
        SEED_MINT_ACCOUNT], &[bump]];

    mint_to(
        ctx::CpiContext::new_with_signer(
        program: token_program.to_account_info(),
        Accounts: MintTo {
          mint: mint_account.to_account_info(),
          to: token_account.to_account_info(),
          mint_authority: mint_account.to_account_info(),
        },
        signer_seeds,
    ), 
        amount,
    ) 

}

pub fn deposit_sol<'info>(
    from: &signer<'info>,
    to: &system_account<'info>,
    system_program: &program<'info,system>,
    amount: u64,

) ->Result<()>{ 
    transfer(
        ctx::CpiContext::new(
            program: system_program.to_account_info(),
            account: transfer{
                from: from.to_account_info(),
                to: to.to_account_info(),
            },
        ),
        lamports: amount,
    )
}