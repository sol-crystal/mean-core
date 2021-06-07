// Register modules
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;

use solana_program::{
    declare_id,
    entrypoint::ProgramResult,
    pubkey::Pubkey
};

use crate::error::StreamError;

declare_id!("2HEkjrj21DX2ecNQjAEUPKwr2pEnwSBjgi9GUHWtKnhH");

pub fn check_program_account(program_id: &Pubkey) -> ProgramResult {
    if program_id != &id() {
        return Err(StreamError::IncorrectProgramId.into());
    }
    Ok(())
}