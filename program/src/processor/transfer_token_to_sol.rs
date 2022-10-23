use solana_program::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult, msg,
    program::invoke, program_pack::Pack, pubkey::Pubkey,
};
use crate::constants::lib::LibConstant;

use crate::errors::TokenTradeError;

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], lamports: u64) -> ProgramResult {
    msg!("Tranfer sol to token, lamports: {}", lamports);
    let accounts_iter = &mut accounts.iter();
    let token_receive = next_account_info(accounts_iter)?;
    let token_receive_address = next_account_info(accounts_iter)?;
    let program = next_account_info(accounts_iter)?;
    let _mint = next_account_info(accounts_iter)?;
    let token_transfer = next_account_info(accounts_iter)?;
    let token_transfer_address = next_account_info(accounts_iter)?;
    let token_program_id = next_account_info(accounts_iter)?;

    msg!("Transfer {} Token lamports from token to Sol", lamports);
    let payer_token_to_vault_ix = spl_token::instruction::transfer(
        &token_program_id.key,
        &token_receive_address.key,
        &token_transfer_address.key,
        &token_receive.key,
        &[],
        lamports,
    )?;

    invoke(
        &payer_token_to_vault_ix,
        &[
            token_program_id.clone(),
            token_receive_address.clone(),
            token_transfer_address.clone(),
            program.clone(),
            token_receive.clone(),
        ],
    )?;

    msg!("Transfer to SOL from token successfully");
    **token_transfer.try_borrow_mut_lamports()? -= lamports / LibConstant::TRANSFER_RATE;
    **token_receive.try_borrow_mut_lamports()? += lamports / LibConstant::TRANSFER_RATE;

    msg!(
        "{} SOL lamports transferred from token",
        lamports / LibConstant::TRANSFER_RATE,
    );

    Ok(())
}
