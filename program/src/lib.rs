mod constants;
mod processors;
mod utils;

use crate::processors::{
    process_abort_if_nefarious, process_adjust_slippage_and_forward, process_report_if_nefarious,
};
use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};

entrypoint!(process_instruction);
// not deployed to mainnet!
pinocchio_pubkey::declare_id!("BfXm7pxBsqF5BpZqKSeNLzBUHXbnvase19ge2XHofhb3");

const ABORT_IF_NEFARIOUS_DISC: u8 = 1;
const ADJUST_SLIPPAGE_VIA_JUPITER_DISC: u8 = 2;
const REPORT_IF_NEFARIOUS_DISC: u8 = 3;

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if program_id != &id() {
        return Err(ProgramError::IncorrectProgramId);
    } else if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }

    let (discriminator, data) =
        instruction_data.split_first().ok_or(ProgramError::InvalidInstructionData)?;

    match *discriminator {
        ABORT_IF_NEFARIOUS_DISC => process_abort_if_nefarious(&data),

        ADJUST_SLIPPAGE_VIA_JUPITER_DISC => process_adjust_slippage_and_forward(accounts, &data),

        REPORT_IF_NEFARIOUS_DISC => process_report_if_nefarious(&data),

        _ => Err(ProgramError::InvalidInstructionData),
    }
}
