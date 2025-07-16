use crate::utils::is_nefarious;
use anti_sandwich_common::NefariousWindow;
use pinocchio::cpi::set_return_data;
use pinocchio::program_error::ProgramError;
use pinocchio::ProgramResult;

enum Report {
    NotNefarious = 0,
    Nefarious = 1,
    Error = 2,
}

impl From<NefariousWindow> for Report {
    fn from(value: NefariousWindow) -> Self {
        match is_nefarious(&value) {
            Ok(nefarious) => {
                if nefarious {
                    Report::Nefarious
                } else {
                    Report::NotNefarious
                }
            }
            Err(_) => Report::Error, // error reading Clock sysvar
        }
    }
}

pub fn process_report_if_nefarious(data: &[u8]) -> ProgramResult {
    let window = NefariousWindow::unpack(data).ok_or(ProgramError::InvalidInstructionData)?;
    let ret: Report = window.into();
    set_return_data(&[ret as u8]);
    Ok(())
}
