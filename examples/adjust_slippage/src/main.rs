mod sample_jupiter_response;
mod simulator;

use crate::sample_jupiter_response::get_sample;
use crate::simulator::Simulator;
use anti_sandwich_sdk::adjust_slippage_at_runtime;
use solana_keypair::Keypair;
use solana_program::pubkey;
use solana_program::pubkey::Pubkey;
use solana_signer::Signer;

const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
const SIM_SLOT: u64 = 353612620;
const SIM_TIMESTAMP: u64 = 1752641622;

const AMOUNT_IN_SOL: f64 = 20.0;
const QUOTE_AMOUNT: f64 = 3421.56;

const BASE_SLIPPAGE: u16 = 600;
const SLIPPAGE_IF_NEFARIOUS: u16 = 200;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    run_single(false).await?;
    run_single(true).await?;

    Ok(())
}

async fn run_single(sim_lands_on_nefarious: bool) -> eyre::Result<()> {
    let mut sim = Simulator::new()?;
    let user = Keypair::new();

    // sample swap routes through 3 AMMs for WSOL -> USDC
    let swap_ix_resp = get_sample(&user.pubkey(), (QUOTE_AMOUNT * 1e6) as u64, BASE_SLIPPAGE)?;

    let mut ixs = swap_ix_resp.compute_budget_instructions;
    ixs.extend(swap_ix_resp.setup_instructions);

    let nefarious_leader_slots = if sim_lands_on_nefarious { vec![SIM_SLOT] } else { vec![] };
    let adjusted_jupiter_tx = adjust_slippage_at_runtime(
        &nefarious_leader_slots,
        SLIPPAGE_IF_NEFARIOUS,
        swap_ix_resp.swap_instruction,
    )?;
    ixs.push(adjusted_jupiter_tx);
    if let Some(cleanup) = swap_ix_resp.cleanup_instruction {
        ixs.push(cleanup);
    }

    sim.set_slot_and_time(SIM_SLOT, SIM_TIMESTAMP);

    let res = sim
        .run(&ixs, &user, AMOUNT_IN_SOL * 2.0, Some(&swap_ix_resp.address_lookup_table_addresses))
        .await?;
    let balance = sim.token_balance(user.pubkey(), USDC_MINT);

    println!("=== Transaction Result ===");
    if sim_lands_on_nefarious {
        println!("Simulated landing on nefarious validator. Slippage was adjusted {}% -> {}% at execution-time", BASE_SLIPPAGE as f64 / 100.0, SLIPPAGE_IF_NEFARIOUS  as f64 / 100.0);
    }
    match res {
        Ok(meta) => {
            println!(
                "Transaction succeeded. {} SOL -> {} USDC",
                AMOUNT_IN_SOL,
                balance as f64 / 1e6
            );
            println!("CUs: {}", meta.compute_units_consumed);
            for log in meta.logs {
                if log.contains("adjusting") {
                    println!("--> {}", log);
                }
            }
        }
        Err(err) => {
            println!("Transaction err was: {}", err.err);
            println!("CUs: {}", err.meta.compute_units_consumed);
            for log in err.meta.logs {
                if log.contains("adjusting") {
                    println!("--> {}", log);
                }
            }
            if sim_lands_on_nefarious {
                println!("(error was expected since in this sim the validator is malicious)");
            }
        }
    }

    println!();

    Ok(())
}
