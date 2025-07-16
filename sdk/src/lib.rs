use anti_sandwich_common::NefariousWindow;
use solana_program::{instruction::Instruction, pubkey, pubkey::Pubkey};

// not deployed to mainnet!
pub const PROGRAM_ID: Pubkey = pubkey!("BfXm7pxBsqF5BpZqKSeNLzBUHXbnvase19ge2XHofhb3");
pub const ABORT_DISC: u8 = 1;
pub const ADJUST_SLIPPAGE_DISC: u8 = 2;

/// Build the 192‑slot NefariousWindow that records which 4‑slot‑leaders are nefarious.
///
/// * `nefarious_leader_slots` – slots where a flagged validator is a leader
///
/// NefariousWindow is always 14 bytes regardless of how many leader slots are nefarious
///
/// Returns an error if a supplied slot is outside the
/// `[baseline_slot, baseline_slot + 191]` range.
fn build_window(nefarious_leader_slots: &[u64]) -> eyre::Result<NefariousWindow> {
    if nefarious_leader_slots.is_empty() {
        return Ok(NefariousWindow::empty());
    }

    let baseline_slot = *nefarious_leader_slots.iter().min().expect("cannot be empty");
    let mut bits = [0u8; 6];

    for &slot in nefarious_leader_slots {
        if !(baseline_slot..=baseline_slot + 191).contains(&slot) {
            return Err(eyre::eyre!(
                "slot {slot} is outside the 192‑slot window starting at {baseline_slot}"
            ));
        }

        let leader = ((slot - baseline_slot) / 4) as usize;
        bits[leader / 8] |= 1 << (leader % 8);
    }

    Ok(NefariousWindow { window_start: baseline_slot, nefarious: bits })
}

pub fn abort_if_nefarious(nefarious_leader_slots: &[u64]) -> eyre::Result<Instruction> {
    let window = build_window(nefarious_leader_slots)?;
    let mut data = Vec::with_capacity(1 + NefariousWindow::LEN);

    data.push(ABORT_DISC);
    data.extend_from_slice(&window.pack_to_vec());

    Ok(Instruction { program_id: PROGRAM_ID, accounts: vec![], data })
}

pub fn adjust_slippage_at_runtime(
    nefarious_leader_slots: &[u64],
    slippage_if_nefarious: u16,
    jupiter_ix: Instruction,
) -> eyre::Result<Instruction> {
    let window = build_window(nefarious_leader_slots)?;
    let mut data = Vec::with_capacity(1 + NefariousWindow::LEN + 2 + jupiter_ix.data.len());

    data.push(ADJUST_SLIPPAGE_DISC);
    data.extend_from_slice(&window.pack_to_vec());
    data.extend_from_slice(&slippage_if_nefarious.to_le_bytes());
    data.extend_from_slice(&jupiter_ix.data);

    Ok(Instruction { program_id: PROGRAM_ID, accounts: jupiter_ix.accounts, data })
}
