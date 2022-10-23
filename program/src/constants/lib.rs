use std::mem::size_of;
use solana_program::pubkey::Pubkey;

pub struct LibConstant {}

impl LibConstant {
    pub const TRANSFER_RATE: u64 = 10;
    pub const TOKEN_TRANSFER_ACCOUNT_LEN: usize = size_of::<Pubkey>() * 3;
}
