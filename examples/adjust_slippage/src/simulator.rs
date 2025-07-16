use eyre::{eyre, OptionExt};
use litesvm::types::TransactionResult;
use litesvm::LiteSVM;
use solana_account::Account;
use solana_instruction::Instruction;
use solana_keypair::Keypair;
use solana_program::address_lookup_table::state::AddressLookupTable;
use solana_program::clock::Clock;
use solana_program::message::{v0, AddressLookupTableAccount, Message, VersionedMessage};
use solana_program::native_token::sol_to_lamports;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use solana_signer::Signer;
use solana_transaction::versioned::VersionedTransaction;
use solana_transaction::Transaction;
use spl_associated_token_account::get_associated_token_address;
use spl_token::state::Account as TokenAccount;
use std::fs;
use std::path::Path;
use std::str::FromStr;

const COMPILED_PROGRAM_PATH: &str = "target/deploy/anti_sandwich_program.so";

pub struct Simulator {
    svm: LiteSVM,
}

impl Simulator {
    pub fn new() -> eyre::Result<Self> {
        if !Path::new(COMPILED_PROGRAM_PATH).exists() {
            return Err(eyre::eyre!(
                "Compiled program not found at {}.\nPlease compile the program first.\n\t`cargo build-sbf --manifest-path program/Cargo.toml`.",
                COMPILED_PROGRAM_PATH
            ));
        }
        let mut svm =
            LiteSVM::new().with_precompiles().with_sysvars().with_builtins().with_sigverify(true);
        svm.add_program_from_file(anti_sandwich_sdk::PROGRAM_ID, COMPILED_PROGRAM_PATH)?;

        let mut sim = Self { svm };
        sim.load_testdata()?;

        Ok(sim)
    }

    pub fn set_slot_and_time(&mut self, slot: u64, time: u64) {
        let mut clock = self.svm.get_sysvar::<Clock>();
        clock.slot = slot;
        clock.unix_timestamp = time as i64;
        self.svm.set_sysvar(&clock);
    }

    pub async fn run(
        &mut self,
        ixs: &[Instruction],
        signer: &Keypair,
        airdrop: f64,
        lookup_tables: Option<&[Pubkey]>,
    ) -> eyre::Result<TransactionResult> {
        self.svm
            .airdrop(&signer.pubkey(), sol_to_lamports(airdrop))
            .map_err(|e| eyre!("failed to airdrop: {}", e.err))?;
        let tx = match lookup_tables {
            None => {
                let tx = Transaction::new(
                    &[&signer],
                    Message::new(ixs, Some(&signer.pubkey())),
                    self.svm.latest_blockhash(),
                );
                self.svm.send_transaction(tx)
            }
            Some(lookup_tables) => {
                let lookup_tables = self.get_all_lookup_tables(lookup_tables)?;

                let msg = VersionedMessage::V0(v0::Message::try_compile(
                    &signer.pubkey(),
                    ixs,
                    &lookup_tables,
                    self.svm.latest_blockhash(),
                )?);

                let tx = VersionedTransaction::try_new(msg, &[signer])?;
                self.svm.send_transaction(tx)
            }
        };

        Ok(tx)
    }

    fn get_all_lookup_tables(
        &self,
        keys: &[Pubkey],
    ) -> eyre::Result<Vec<AddressLookupTableAccount>> {
        let mut lookup_tables = Vec::new();

        for key in keys {
            let account = self
                .svm
                .get_account(key)
                .ok_or(eyre::eyre!("ALT {} not found in account states", key))?;
            if let Ok(lookup_table) = AddressLookupTable::deserialize(&account.data) {
                if !lookup_table.addresses.is_empty() {
                    lookup_tables.push(AddressLookupTableAccount {
                        key: *key,
                        addresses: lookup_table.addresses.into_owned(),
                    });
                };
            }
        }

        Ok(lookup_tables)
    }

    pub fn token_balance(&self, user: Pubkey, mint: Pubkey) -> u64 {
        let ata = get_associated_token_address(&user, &mint);
        let Some(account) = self.svm.get_account(&ata) else {
            return 0;
        };
        let state = TokenAccount::unpack(&account.data).ok().unwrap_or_default();
        state.amount
    }

    fn load_testdata(&mut self) -> eyre::Result<()> {
        let testdata_path = Path::new("examples/testdata");
        if !testdata_path.exists() {
            return Err(eyre!("examples/testdata directory not found"));
        }

        for entry in fs::read_dir(testdata_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let filename = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .ok_or_eyre("Invalid filename")?;

                if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                    match extension {
                        "json" => {
                            let address = filename
                                .strip_suffix(".json")
                                .ok_or_eyre("Invalid JSON filename")?;
                            let file_content = fs::read_to_string(&path)?;
                            let account: Account = serde_json::from_str(&file_content)?;
                            self.svm.set_account(Pubkey::from_str(address)?, account)?;
                        }
                        "so" => {
                            let address =
                                filename.strip_suffix(".so").ok_or_eyre("Invalid SO filename")?;
                            self.svm.add_program_from_file(Pubkey::from_str(address)?, &path)?;
                        }
                        _ => {
                            continue;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
