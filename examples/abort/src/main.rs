use anti_sandwich_sdk::abort_if_nefarious;
use eyre::eyre;
use litesvm::types::FailedTransactionMetadata;
use litesvm::LiteSVM;
use solana_instruction::error::InstructionError;
use solana_keypair::Keypair;
use solana_program::instruction::Instruction;
use solana_program::message::Message;
use solana_program::native_token::sol_to_lamports;
use solana_signer::Signer;
use solana_transaction::Transaction;
use solana_transaction_error::TransactionError;
use std::path::Path;

const COMPILED_PROGRAM_PATH: &str = "target/deploy/anti_sandwich_program.so";

pub fn main() -> eyre::Result<()> {
    if !Path::new(COMPILED_PROGRAM_PATH).exists() {
        return Err(eyre::eyre!(
                "Compiled program not found at {}.\nPlease compile the program first.\n\t`cargo build-sbf --manifest-path program/Cargo.toml`.",
                COMPILED_PROGRAM_PATH
            ));
    }
    let mut svm =
        LiteSVM::new().with_precompiles().with_sysvars().with_builtins().with_sigverify(true);
    svm.add_program_from_file(anti_sandwich_sdk::PROGRAM_ID, COMPILED_PROGRAM_PATH)?;

    let ix = abort_if_nefarious(&[350_000_012])?;

    // ==== normal scenario (validator isn't known to be malicious) ====
    run(&mut svm, &ix, 350_000_000, false)?;

    // ==== abort scenario (validator is malicious) ====
    run(&mut svm, &ix, 350_000_013, true)?;

    Ok(())
}

fn run(svm: &mut LiteSVM, ix: &Instruction, slot: u64, expect_abort: bool) -> eyre::Result<()> {
    let user = Keypair::new();
    svm.airdrop(&user.pubkey(), sol_to_lamports(1.0))
        .map_err(|e| eyre!("couldnt airdrop: {}", e.err))?;

    svm.warp_to_slot(slot);

    let res = svm.send_transaction(Transaction::new(
        &[&user],
        Message::new(&[ix.clone()], Some(&user.pubkey())),
        svm.latest_blockhash(),
    ));

    match (res, expect_abort) {
        (Ok(res), true) => {
            println!("!! tx {} succeeded but expected abort !!", res.signature)
        }
        (Ok(res), false) => {
            println!(
                "tx {} succeeded, as expected. CU={}",
                res.signature, res.compute_units_consumed
            )
        }
        (Err(err), true) => {
            println!(
                "tx {} aborted (errno={}), as expected. CU={}",
                err.meta.signature,
                custom_err_str(&err),
                err.meta.compute_units_consumed
            )
        }
        (Err(err), false) => {
            println!("!! tx {} aborted but expected success !!", err.meta.signature)
        }
    }

    Ok(())
}

fn custom_err_str(meta: &FailedTransactionMetadata) -> String {
    match meta.err {
        TransactionError::InstructionError(_, InstructionError::Custom(v)) => {
            format!("{v}")
        }
        _ => meta.err.to_string(),
    }
}
