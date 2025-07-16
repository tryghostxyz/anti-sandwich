/// `NefariousWindow` encodes information about which upcoming validators are considered malicious,
/// in a compact 14-byte format suitable for efficient transmission.
///
/// - `window_start`: the first slot in the 192-slot window this struct covers.
/// - `nefarious`: a 48-bit bitmap (6 bytes), where each bit corresponds to one 4-slot leader chunk,
///   indicating whether that leader is marked nefarious (bit set to 1).
///
/// The 192-slot window is divided into 48 leader chunks (4 slots per leader), and this structure
/// allows constant-time querying of whether a given slot falls into a "nefarious" leader chunk.
///
/// This compact encoding avoids sending a large list of slots, instead using a bitmap to represent
/// 48 leaders efficiently. It is intended for fast deserialization, efficient checks,
/// and small payloads
///
/// The window size of 192 slots was chosen to provide sufficient lookahead for
/// clients. A Solana transaction is valid for up to 151 slots (depending on blockhash used)
/// The 192-slot window provides this coverage plus additional slack to accommodate
/// potential client-side caching of this `NefariousWindow`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct NefariousWindow {
    pub window_start: u64,
    pub nefarious: [u8; 6],
}

impl NefariousWindow {
    /// 8 (u64) + 6 ([u8; 6])
    pub const LEN: usize = 14;

    /// 192 slots / 4 slots per leader
    pub const MAX_LEADERS: usize = 48;

    #[inline(always)]
    pub fn unpack(input: &[u8]) -> Option<Self> {
        if input.len() != Self::LEN {
            return None;
        }
        let mut start = [0u8; 8];
        start.copy_from_slice(&input[0..8]);
        let window_start = u64::from_le_bytes(start);

        let mut nefarious = [0u8; 6];
        nefarious.copy_from_slice(&input[8..14]);

        Some(Self { window_start, nefarious })
    }

    pub fn pack(&self, dst: &mut [u8]) -> Option<()> {
        if dst.len() < Self::LEN {
            return None;
        }
        dst[0..8].copy_from_slice(&self.window_start.to_le_bytes());
        dst[8..14].copy_from_slice(&self.nefarious);
        Some(())
    }

    pub fn pack_to_vec(&self) -> Vec<u8> {
        let mut data = vec![0; Self::LEN];
        self.pack(&mut data).expect("pack should never fail with correctly sized buffer");
        data
    }

    /// True if the 4-slot chunk that contains `slot` is marked nefarious.
    #[inline(always)]
    pub fn is_nefarious(&self, slot: u64) -> bool {
        if slot < self.window_start {
            return false;
        }
        let leader = slot.saturating_sub(self.window_start) / 4;
        if leader >= Self::MAX_LEADERS as u64 {
            return false;
        }
        let byte = self.nefarious[leader as usize / 8];
        (byte >> (leader & 7)) & 1 != 0
    }

    /// Inclusive slot range `[first, last]` for which `is_nefarious`
    /// returns meaningful results (the 192 slots this struct covers).
    #[inline(always)]
    pub fn valid_land_range(&self) -> core::ops::RangeInclusive<u64> {
        self.window_start..=self.window_start + 191
    }

    pub fn empty() -> NefariousWindow {
        NefariousWindow { window_start: 0, nefarious: [0; 6] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn set_bit(bits: &mut [u8; 6], leader: usize) {
        assert!(leader < NefariousWindow::MAX_LEADERS);
        bits[leader / 8] |= 1 << (leader % 8);
    }

    #[test]
    fn empty() {
        let win = NefariousWindow { window_start: 350_000_000, nefarious: [0; 6] };
        for slot in win.valid_land_range() {
            assert!(!win.is_nefarious(slot));
        }
    }

    #[test]
    fn out_of_range() {
        let start = 350_000_000;
        let mut bits = [0u8; 6];
        // mark offsets 0, 10 and 20  (slots 0-3, 40-43 and 80-83)
        set_bit(&mut bits, 0);
        set_bit(&mut bits, 10);
        set_bit(&mut bits, 20);

        let win = NefariousWindow { window_start: start, nefarious: bits };

        // inside range
        assert!(win.is_nefarious(start));
        assert!(win.is_nefarious(start + 2));
        assert!(!win.is_nefarious(start + 4));
        assert!(win.is_nefarious(start + 40));
        assert!(win.is_nefarious(start + 43));
        assert!(win.is_nefarious(start + 80));
        assert!(win.is_nefarious(start + 83));
        assert!(!win.is_nefarious(start + 85));

        // below range
        assert!(!win.is_nefarious(start - 1));
        assert!(!win.is_nefarious(start - 100));

        // above range
        assert!(!win.is_nefarious(start + 192));
        assert!(!win.is_nefarious(start + 200));
    }

    #[test]
    fn all_set() {
        let win = NefariousWindow { window_start: 350_000_000, nefarious: [0xFF; 6] };
        for slot in win.valid_land_range() {
            assert!(win.is_nefarious(slot), "slot {slot}");
        }
    }

    #[test]
    fn boundary() {
        let mut bits = [0u8; 6];
        set_bit(&mut bits, 0); // 350_000_000..350_000_003
        set_bit(&mut bits, 47); // 350_000_188..350_000_191

        let win = NefariousWindow { window_start: 350_000_000, nefarious: bits };

        assert!(win.is_nefarious(350_000_000));
        assert!(win.is_nefarious(350_000_003));
        assert!(!win.is_nefarious(350_000_004));

        assert!(win.is_nefarious(350_000_188));
        assert!(win.is_nefarious(350_000_191));
        assert!(!win.is_nefarious(350_000_192));
    }

    #[test]
    fn round_trip() {
        let original = NefariousWindow {
            window_start: 350_000_042,
            nefarious: [0b1010_1010, 0b0000_0111, 0, 0, 0, 0],
        };
        let mut buf = [0u8; 14];
        original.pack(&mut buf).unwrap();
        let decoded = NefariousWindow::unpack(&buf).unwrap();
        assert_eq!(original, decoded);
    }

    proptest! {
        #[test]
        fn prop_pack_unpack(start in 350_000_000u64..360_000_000,
                            leaders in prop::collection::vec(0usize..NefariousWindow::MAX_LEADERS, 0..=NefariousWindow::MAX_LEADERS)) {
            let mut nefarious = [0u8; 6];
            for &i in &leaders {
                set_bit(&mut nefarious, i);
            }
            let original = NefariousWindow { window_start: start, nefarious };
            let mut buf = [0u8; 14];
            original.pack(&mut buf).unwrap();
            let decoded = NefariousWindow::unpack(&buf).unwrap();
            assert_eq!(original, decoded);
        }

        #[test]
        fn prop_slot_mapping(start in 350_000_000u64..360_000_000,
                             leaders in prop::collection::vec(0usize..NefariousWindow::MAX_LEADERS, 0..=15)) {
            let mut nefarious = [0u8; 6];
            for &i in &leaders {
                set_bit(&mut nefarious, i);
            }
            let win = NefariousWindow { window_start: start, nefarious };
            for slot in win.valid_land_range() {
                let leader = ((slot - start) / 4) as usize;
                assert_eq!(win.is_nefarious(slot), leaders.contains(&leader));
            }
        }
    }
}
