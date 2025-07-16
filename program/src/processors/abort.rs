use crate::utils::is_nefarious;
use anti_sandwich_common::NefariousWindow;
use pinocchio::program_error::ProgramError;
use pinocchio::ProgramResult;

pub fn process_abort_if_nefarious(data: &[u8]) -> ProgramResult {
    let window = NefariousWindow::unpack(data).ok_or(ProgramError::InvalidInstructionData)?;
    if is_nefarious(&window)? {
        Err(ProgramError::Custom(100))
    } else {
        Ok(())
    }
}
