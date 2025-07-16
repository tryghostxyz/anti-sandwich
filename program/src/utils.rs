use anti_sandwich_common::NefariousWindow;
use pinocchio::program_error::ProgramError;
use pinocchio::sysvars::Sysvar;

#[inline(always)]
pub(crate) fn is_nefarious(window: &NefariousWindow) -> Result<bool, ProgramError> {
    let clock = pinocchio::sysvars::clock::Clock::get()?;
    Ok(window.is_nefarious(clock.slot))
}
