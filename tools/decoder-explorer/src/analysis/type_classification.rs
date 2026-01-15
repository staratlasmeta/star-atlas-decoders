use super::{AccountTypeIdentifier, AnalysisContext};
use anyhow::Result;
use carbon_core::account::AccountDecoder;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

struct DecodeFailure {
    pubkey: Pubkey,
    data_size: usize,
    owner: Pubkey,
    discriminator_preview: String,
}

pub fn analyze<'a, D, T>(ctx: &AnalysisContext<'a>, decoder: &D) -> Result<()>
where
    D: AccountDecoder<'a, AccountType = T>,
    T: AccountTypeIdentifier,
{
    tracing::info!("\n=== Account Type Classification ===");

    let total_accounts = ctx.total_accounts();
    let mut type_counts: HashMap<String, usize> = HashMap::new();
    let mut failed_accounts = Vec::new();

    for (pubkey, account) in ctx.accounts {
        match decoder.decode_account(account) {
            Some(decoded) => {
                let type_name = decoded.data.type_name().to_string();
                *type_counts.entry(type_name).or_insert(0) += 1;
            }
            None => {
                let preview_len = account.data.len().min(16);
                let discriminator_preview = account.data[..preview_len]
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<_>>()
                    .join(" ");

                failed_accounts.push(DecodeFailure {
                    pubkey: *pubkey,
                    data_size: account.data.len(),
                    owner: account.owner,
                    discriminator_preview,
                });
            }
        }
    }

    let decode_failures = failed_accounts.len();
    tracing::info!("Total accounts: {}", total_accounts);
    tracing::info!(
        "Successfully decoded: {}",
        total_accounts - decode_failures
    );
    tracing::info!("Decode failures: {}", decode_failures);

    if decode_failures > 0 {
        let failure_rate = (decode_failures as f64 / total_accounts as f64) * 100.0;
        tracing::error!("Decode failure rate: {:.2}%", failure_rate);
    }

    // Sort by count descending
    let mut sorted_types: Vec<_> = type_counts.iter().collect();
    sorted_types.sort_by(|a, b| b.1.cmp(a.1));

    tracing::info!("\nAccount Type Distribution:");
    for (type_name, count) in sorted_types {
        let percentage = (*count as f64 / total_accounts as f64) * 100.0;
        tracing::info!("  {:30} {:8} ({:5.2}%)", type_name, count, percentage);
    }

    // Report failures with debug info
    if !failed_accounts.is_empty() {
        tracing::error!("\n=== Decode Failures ===");
        for (i, failure) in failed_accounts.iter().enumerate() {
            tracing::error!("[{}] Account: {}", i + 1, failure.pubkey);
            tracing::error!("    Owner: {}", failure.owner);
            tracing::error!("    Data size: {} bytes", failure.data_size);
            tracing::error!("    First 16 bytes: {}", failure.discriminator_preview);
        }
    }

    Ok(())
}
