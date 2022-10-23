use crate::errors::TokenTradeError;
use crate::instruction::TokenTradeInstruction;
use solana_program::decode_error::DecodeError;
use solana_program::program_error::PrintProgramError;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};
pub mod initialize;
pub mod transfer_sol_to_token;
pub mod transfer_token_to_sol;

pub struct Processor {}
impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Process instructions");
        let instruction = TokenTradeInstruction::unpack(instruction_data)?;
        match instruction {
            TokenTradeInstruction::Initialize => {
                msg!("Initialize...");
                initialize::process(program_id, accounts)?;
            }

            TokenTradeInstruction::TransferSolToToken { data } => {
                msg!("Transfer token to SOL");
                transfer_sol_to_token::process(program_id, accounts, data)?;
            }

            TokenTradeInstruction::TransferTokenToSol { data } => {
                msg!("Transfer SOL to token");
                transfer_token_to_sol::process(program_id, accounts, data)?;
            }
        }

        Ok(())
    }
}

impl PrintProgramError for TokenTradeError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(&self.to_string());
    }
}
