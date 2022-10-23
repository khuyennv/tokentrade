use solana_program::{
    account_info::next_account_info,
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_associated_token_account::solana_program::system_instruction;

use crate::errors::TokenTradeError;
use crate::constants::lib::LibConstant;

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], lamports: u64) -> ProgramResult {
    msg!("Transfer sol to token, lamports: {}", lamports);
    let accounts_iter = &mut accounts.iter();
    let token_transfer = next_account_info(accounts_iter)?;
    let token_transfer_address = next_account_info(accounts_iter)?;
    let _program = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let token_receive = next_account_info(accounts_iter)?;
    let token_receive_address = next_account_info(accounts_iter)?;
    let token_program_id = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let (token_receive_from_address, vault_bump_seed) =
        Pubkey::find_program_address(&[b"vault", mint.key.as_ref()], program_id);

    if token_receive_from_address != *token_receive.key {
        msg!("Invalid vault account");
        return Err(TokenTradeError::InvalidAccountAddress.into());
    }

    msg!("Transfer SOL from token to program");
    let instruction = system_instruction::transfer(token_transfer.key, token_receive.key, lamports);
    let required_accounts_take_sol = [system_program.clone(), token_transfer.clone(), token_receive.clone()];

    invoke(&instruction, &required_accounts_take_sol);

    // send token
    msg!("Transfer token");
    let transfer_token_to_payer_ix = spl_token::instruction::transfer(
        token_program_id.key,
        &token_receive_address.key,
        &token_transfer_address.key,
        &token_receive.key,
        &[],
        lamports * LibConstant::TRANSFER_RATE,
    )?;

    msg!("Transfer SOL from token successfully");

    invoke_signed(
        &transfer_token_to_payer_ix,
        &[
            token_program_id.clone(),
            token_transfer_address.clone(),
            token_receive_address.clone(),
            token_receive.clone(),
        ],
        &[&[b"vault", mint.key.as_ref(), &[vault_bump_seed]]],
    )?;

    Ok(())
}
