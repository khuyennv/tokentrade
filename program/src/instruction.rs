use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::errors::TokenTradeError::InvalidInstruction;

pub enum TokenTradeInstruction {
    Initialize,
    TransferSolToToken { data: u64 },
    TransferTokenToSol { data: u64 },
}

impl TokenTradeInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        return match tag {
            0 => Ok(Self::Initialize),
            1 => Ok(Self::TransferSolToToken {
                data: Self::parse(rest)?,
            }),
            2 => Ok(Self::TransferTokenToSol {
                data: Self::parse(rest)?,
            }),
            _ => Err(InvalidInstruction.into()),
        };
    }

    fn parse(input: &[u8]) -> Result<u64, ProgramError> {
        let data = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;

        return Ok(data);
    }
}
