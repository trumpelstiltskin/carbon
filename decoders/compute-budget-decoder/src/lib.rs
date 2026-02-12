#![no_std]

extern crate alloc;
use solana_pubkey::Pubkey;

pub struct ComputeBudgetDecoder;

pub mod instructions;

pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("ComputeBudget111111111111111111111111111111");
