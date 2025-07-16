use jupiter_swap_api_client::swap::PrioritizationType::ComputeBudget;
use jupiter_swap_api_client::swap::SwapInstructionsResponse;
use solana_instruction::{AccountMeta, Instruction};
use solana_program::pubkey;
use solana_program::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;
use std::str::FromStr;

const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
const WSOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

fn addr(s: &str) -> eyre::Result<Pubkey> {
    Ok(Pubkey::from_str(s)?)
}

pub fn get_sample(
    user: &Pubkey,
    quote_amount: u64,
    slippage: u16,
) -> eyre::Result<SwapInstructionsResponse> {
    let wsol_ata = get_associated_token_address(user, &WSOL_MINT);
    let usdc_ata = get_associated_token_address(user, &USDC_MINT);

    let mut resp = SwapInstructionsResponse {
        token_ledger_instruction: None,
        compute_budget_instructions: vec![
            Instruction {
                program_id: addr("ComputeBudget111111111111111111111111111111")?,
                accounts: vec![],
                data: vec![2, 0, 53, 12, 0],
            },
            Instruction {
                program_id: addr("ComputeBudget111111111111111111111111111111")?,
                accounts: vec![],
                data: vec![3, 4, 23, 1, 0, 0, 0, 0, 0],
            },
        ],
        setup_instructions: vec![
            Instruction {
                program_id: addr("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")?,
                accounts: vec![
                    AccountMeta { pubkey: *user, is_signer: true, is_writable: true },
                    AccountMeta { pubkey: wsol_ata, is_signer: false, is_writable: true },
                    AccountMeta { pubkey: *user, is_signer: false, is_writable: false },
                    AccountMeta { pubkey: WSOL_MINT, is_signer: false, is_writable: false },
                    AccountMeta {
                        pubkey: addr("11111111111111111111111111111111")?,
                        is_signer: false,
                        is_writable: false,
                    },
                    AccountMeta {
                        pubkey: addr("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?,
                        is_signer: false,
                        is_writable: false,
                    },
                ],
                data: vec![1],
            },
            Instruction {
                program_id: addr("11111111111111111111111111111111")?,
                accounts: vec![
                    AccountMeta { pubkey: *user, is_signer: true, is_writable: true },
                    AccountMeta { pubkey: wsol_ata, is_signer: false, is_writable: true },
                ],
                data: vec![2, 0, 0, 0, 0, 200, 23, 168, 4, 0, 0, 0],
            },
            Instruction {
                program_id: addr("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?,
                accounts: vec![AccountMeta {
                    pubkey: wsol_ata,
                    is_signer: false,
                    is_writable: true,
                }],
                data: vec![17],
            },
            Instruction {
                program_id: addr("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")?,
                accounts: vec![
                    AccountMeta { pubkey: *user, is_signer: true, is_writable: true },
                    AccountMeta { pubkey: usdc_ata, is_signer: false, is_writable: true },
                    AccountMeta { pubkey: *user, is_signer: false, is_writable: false },
                    AccountMeta { pubkey: USDC_MINT, is_signer: false, is_writable: false },
                    AccountMeta {
                        pubkey: addr("11111111111111111111111111111111")?,
                        is_signer: false,
                        is_writable: false,
                    },
                    AccountMeta {
                        pubkey: addr("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?,
                        is_signer: false,
                        is_writable: false,
                    },
                ],
                data: vec![1],
            },
        ],
        swap_instruction: Instruction {
            program_id: addr("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4")?,
            accounts: vec![
                AccountMeta {
                    pubkey: addr("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("2MFoS3MPtvyQ4Wh4M9pdfPjz6UhVoNbFbGJAskCPCj3h")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta { pubkey: *user, is_signer: true, is_writable: false },
                AccountMeta { pubkey: wsol_ata, is_signer: false, is_writable: true },
                AccountMeta {
                    pubkey: addr("H1qQ6Hent1C5wa4Hc3GK2V1sgg4grvDBbmKd5H8dsTmo")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("JSvtokJbtGsYhneKomFBjnJh4djEQLdHV2kAeS43bBZ")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta { pubkey: usdc_ata, is_signer: false, is_writable: true },
                AccountMeta {
                    pubkey: addr("So11111111111111111111111111111111111111112")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("D8cy77BBepLMngZx6ZukaTff5hCt1HrWyKk3Hnd9oitf")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("CMfBovCEj8zQgZDSc8iQYYP9r8JMFsnTKED1W55d9ghy")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("HyGVf4UhoQ4ux9ueZgTCf6aJwCcvWqeWf258ZtbeRteV")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("2MFoS3MPtvyQ4Wh4M9pdfPjz6UhVoNbFbGJAskCPCj3h")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("H1qQ6Hent1C5wa4Hc3GK2V1sgg4grvDBbmKd5H8dsTmo")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("F3Hej7eXZMWTr141APvCiNM7ZUn22cD9b1p8EcfFH8vn")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("DHHd8MrNU6YBrfJw47dyzNuuWkr4YkWrjkt5MWVUZvUN")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("4BoHhThRWuokMQz5kER3SJoEGKPHkS68dCsGgJdfYJJo")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("FYidwukt6P8dmwKUbxNUihDbLhBmvenGeDkV8s1yxKq9")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("7VRs5J28yYXTjjq8NSKbXoamcbe5qHGVMGbEyLQMLB31")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("8RVPH46opPd3qLy1n1djntzGMZxnqEzbYs9uoeixdnwk")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("8RVPH46opPd3qLy1n1djntzGMZxnqEzbYs9uoeixdnwk")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("3ZDBff7jeQaksmGvmkRix36rU159EBDjYiPThvV8QVZM")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("6e9hhj57RYhgHq9GcwMGnd6XmPR6ADi7NUyAejz75LVr")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("5zvhFRN45j9oePohUQ739Z4UaSrgPoJ8NLaS2izFuX1j")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("2MFoS3MPtvyQ4Wh4M9pdfPjz6UhVoNbFbGJAskCPCj3h")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("H1qQ6Hent1C5wa4Hc3GK2V1sgg4grvDBbmKd5H8dsTmo")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("F3Hej7eXZMWTr141APvCiNM7ZUn22cD9b1p8EcfFH8vn")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("3AoQxt2k9PYcesFxYWdKSBpJpPtmDZEXdJpw8SgJLJMb")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("F7SK4t9JXn9Wx9YyKHqSECTSk3dYja2UdGXsyzxJwcdk")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("Cas7mdvJurYRD7gRa4pjXCYVngyuwBbmArVnxrUzX4JZ")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("7jt7KQ9ZDybYCSk7Mf7XpAad7Mc7NQbSzShFo2FU13Bd")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("8RVPH46opPd3qLy1n1djntzGMZxnqEzbYs9uoeixdnwk")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("8RVPH46opPd3qLy1n1djntzGMZxnqEzbYs9uoeixdnwk")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("3ZDBff7jeQaksmGvmkRix36rU159EBDjYiPThvV8QVZM")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("SoLFiHG9TfgtdUXUjWAxi3LtvYuFyDLVhBWxdMZxyCe")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("2MFoS3MPtvyQ4Wh4M9pdfPjz6UhVoNbFbGJAskCPCj3h")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("AxHocY4moH8roYQXMQWqoehtW5piMtTJQYmfL4wQ83D8")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("FfcnazsC12gejkhp4gY96Jb9RYRMsnCDqsbeuQYknUKi")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("9dWWzz1eLTKX5tuHBQT8qexq3tskdnsqaDudoNrEt7TJ")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("F3Hej7eXZMWTr141APvCiNM7ZUn22cD9b1p8EcfFH8vn")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("JSvtokJbtGsYhneKomFBjnJh4djEQLdHV2kAeS43bBZ")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("Sysvar1nstructions1111111111111111111111111")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("obriQD1zbpyLz95G5n7nJe6a4DPjpFwa5XYPoNm113y")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("BWBHrYqfcjAh5dSiRwzPnY4656cApXVXmkeDmAfwBKQG")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("GZsNmWKbqhMYtdSkkvMdEyQF9k5mLmP7tTKYWZjcHVPE")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("6YawcNeZ74tRyCv4UfGydYMr7eho7vbUR6ScVffxKAb3")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("C3tPQ8TRcHybnPpR8KMASUVD3PukQRRHEsLwxorJMhgm")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("AAamGhyPfpQJWfZHTq944NM1cFvoVLDrQxt7HGjeRQUS")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("JSvtokJbtGsYhneKomFBjnJh4djEQLdHV2kAeS43bBZ")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("F3Hej7eXZMWTr141APvCiNM7ZUn22cD9b1p8EcfFH8vn")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("J4HJYz4p7TRP96WVFky3vh7XryxoFehHjoRySUTeSeXw")?,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: addr("J4HJYz4p7TRP96WVFky3vh7XryxoFehHjoRySUTeSeXw")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("J4HJYz4p7TRP96WVFky3vh7XryxoFehHjoRySUTeSeXw")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("2MFoS3MPtvyQ4Wh4M9pdfPjz6UhVoNbFbGJAskCPCj3h")?,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: addr("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?,
                    is_signer: false,
                    is_writable: false,
                },
            ],
            data: vec![
                193, 32, 155, 51, 65, 214, 156, 129, 1, 4, 0, 0, 0, 25, 14, 0, 2, 25, 86, 0, 2, 61,
                0, 42, 2, 4, 58, 0, 58, 2, 4, 0, 200, 23, 168, 4, 0, 0, 0, 125, 167, 130, 193, 0,
                0, 0, 0, 16, 39, 0,
            ],
        },
        cleanup_instruction: Some(Instruction {
            program_id: addr("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?,
            accounts: vec![
                AccountMeta { pubkey: wsol_ata, is_signer: false, is_writable: true },
                AccountMeta { pubkey: *user, is_signer: false, is_writable: true },
                AccountMeta { pubkey: *user, is_signer: true, is_writable: false },
            ],
            data: vec![9],
        }),
        other_instructions: vec![],
        address_lookup_table_addresses: vec![
            addr("254RB8oNnV2sGphrjgggwNWTk97LbP82RJXneSmejqSh")?,
            addr("2YZvo6LkePK8V2G2ZaS8UxBYX2Ph6udCu5iuaYAqVM38")?,
            addr("58G1beNP7YZwA1sHD3MTUaY3YxbYmZxntqk8Xb9tC9e4")?,
            addr("6LpcMQrSj6hJc176rU1sdHdAzja4a9xaDgjRuGxfP3oH")?,
        ],
        prioritization_fee_lamports: 99999,
        compute_unit_limit: 800000,
        prioritization_type: Some(ComputeBudget {
            micro_lamports: 71428,
            estimated_micro_lamports: Some(154966),
        }),
        dynamic_slippage_report: None,
        simulation_error: None,
    };

    let slippage_bytes = slippage.to_le_bytes();
    let quote_bytes = quote_amount.to_le_bytes();

    let data_len = resp.swap_instruction.data.len();
    resp.swap_instruction.data[data_len - 3] = slippage_bytes[0];
    resp.swap_instruction.data[data_len - 2] = slippage_bytes[1];

    resp.swap_instruction.data[data_len - 11..data_len - 3].copy_from_slice(&quote_bytes);

    Ok(resp)
}
