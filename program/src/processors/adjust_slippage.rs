use crate::constants::JUPITER_V6;
use crate::utils::is_nefarious;
use anti_sandwich_common::NefariousWindow;
use pinocchio::account_info::AccountInfo;
use pinocchio::cpi::invoke_unchecked;
use pinocchio::instruction::{AccountMeta, Instruction};
use pinocchio::program_error::ProgramError;
use pinocchio::ProgramResult;
use pinocchio_log::log;

const ROUTE_DISC: [u8; 8] = [229, 23, 203, 151, 122, 227, 173, 42];
const ROUTE_WITH_TL_DISC: [u8; 8] = [150, 86, 71, 116, 167, 93, 14, 104];
const SHARED_ACCOUNTS_ROUTE_DISC: [u8; 8] = [193, 32, 155, 51, 65, 214, 156, 129];
const SHARED_ACCOUNTS_ROUTE_WITH_TL_DISC: [u8; 8] = [230, 121, 143, 80, 119, 159, 106, 170];

const MIN_JUPITER_DATA_LEN: usize = 27; // sanity-check only. jup6 responsible for full validation

pub fn process_adjust_slippage_and_forward(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    if data.len() < NefariousWindow::LEN + 2 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let window = NefariousWindow::unpack(&data[..NefariousWindow::LEN])
        .ok_or(ProgramError::InvalidInstructionData)?;
    let new_slippage_bps =
        u16::from_le_bytes([data[NefariousWindow::LEN], data[NefariousWindow::LEN + 1]]);

    let should_adjust_slippage = is_nefarious(&window)?;

    let jupiter_data = &data[NefariousWindow::LEN + 2..];
    if jupiter_data.len() < MIN_JUPITER_DATA_LEN {
        return Err(ProgramError::InvalidInstructionData);
    }
    let disc = &jupiter_data[..8];
    if disc != ROUTE_DISC
        && disc != ROUTE_WITH_TL_DISC
        && disc != SHARED_ACCOUNTS_ROUTE_DISC
        && disc != SHARED_ACCOUNTS_ROUTE_WITH_TL_DISC
    {
        return Err(ProgramError::InvalidInstructionData);
    }

    log!("adjusting slippage? = {}", if should_adjust_slippage { "yes" } else { "no" },);

    if should_adjust_slippage {
        log!("adjusting slippage to {} bps", new_slippage_bps);
        let mutable_data = unsafe {
            std::slice::from_raw_parts_mut(jupiter_data.as_ptr() as *mut u8, jupiter_data.len())
        };

        let tail = jupiter_data.len() - 3; // [slippage:u16, fee:u8]
        mutable_data[tail..tail + 2].copy_from_slice(&new_slippage_bps.to_le_bytes());
    }

    // TODO: more efficient way of passing account info to ix?
    let account_metas: Vec<AccountMeta> = accounts
        .iter()
        .map(|ai| AccountMeta {
            pubkey: ai.key(),
            is_signer: ai.is_signer(),
            is_writable: ai.is_writable(),
        })
        .collect();
    let ix = Instruction { program_id: &JUPITER_V6, accounts: &account_metas, data: jupiter_data };
    let accounts = accounts.iter().map(|ai| ai.into()).collect::<Vec<_>>();

    unsafe { invoke_unchecked(&ix, &accounts) };
    Ok(())
}
