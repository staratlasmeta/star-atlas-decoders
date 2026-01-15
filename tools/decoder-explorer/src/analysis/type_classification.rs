use super::{AccountTypeIdentifier, AnalysisContext};
use anyhow::Result;
use carbon_core::account::AccountDecoder;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

const MAX_SAMPLE_PUBKEYS: usize = 5;

/// Key for grouping similar failures: discriminator (first 8 bytes) + data size
#[derive(Hash, Eq, PartialEq, Clone)]
struct FailureSignature {
    discriminator: [u8; 8],
    data_size: usize,
}

struct FailureGroup {
    discriminator_preview: String,
    data_size: usize,
    pubkeys: Vec<Pubkey>,
}

pub fn analyze<'a, D, T>(ctx: &AnalysisContext<'a>, decoder: &D) -> Result<()>
where
    D: AccountDecoder<'a, AccountType = T>,
    T: AccountTypeIdentifier,
{
    tracing::info!("\n=== Account Type Classification ===");

    let total_accounts = ctx.total_accounts();
    let mut type_counts: HashMap<String, usize> = HashMap::new();
    let mut failure_groups: HashMap<FailureSignature, FailureGroup> = HashMap::new();

    for (pubkey, account) in ctx.accounts {
        match decoder.decode_account(account) {
            Some(decoded) => {
                let type_name = decoded.data.type_name().to_string();
                *type_counts.entry(type_name).or_insert(0) += 1;
            }
            None => {
                let mut discriminator = [0u8; 8];
                let copy_len = account.data.len().min(8);
                discriminator[..copy_len].copy_from_slice(&account.data[..copy_len]);

                let signature = FailureSignature {
                    discriminator,
                    data_size: account.data.len(),
                };

                let group = failure_groups.entry(signature).or_insert_with(|| {
                    let preview_len = account.data.len().min(16);
                    let discriminator_preview = account.data[..preview_len]
                        .iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<Vec<_>>()
                        .join(" ");

                    FailureGroup {
                        discriminator_preview,
                        data_size: account.data.len(),
                        pubkeys: Vec::new(),
                    }
                });

                group.pubkeys.push(*pubkey);
            }
        }
    }

    let decode_failures: usize = failure_groups.values().map(|g| g.pubkeys.len()).sum();
    tracing::info!("Total accounts: {}", total_accounts);
    tracing::info!("Successfully decoded: {}", total_accounts - decode_failures);
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

    // Report aggregated failures
    if !failure_groups.is_empty() {
        tracing::error!(
            "\n=== Decode Failures ({} unique signatures) ===",
            failure_groups.len()
        );

        // Sort groups by count descending
        let mut sorted_groups: Vec<_> = failure_groups.into_iter().collect();
        sorted_groups.sort_by(|a, b| b.1.pubkeys.len().cmp(&a.1.pubkeys.len()));

        for (i, (_, group)) in sorted_groups.iter().enumerate() {
            let count = group.pubkeys.len();
            tracing::error!(
                "\n[{}] {} account{} (size: {} bytes)",
                i + 1,
                count,
                if count == 1 { "" } else { "s" },
                group.data_size
            );
            tracing::error!("    First 16 bytes: {}", group.discriminator_preview);

            let sample_count = count.min(MAX_SAMPLE_PUBKEYS);
            tracing::error!("    Sample addresses ({}/{}):", sample_count, count);
            for pubkey in group.pubkeys.iter().take(MAX_SAMPLE_PUBKEYS) {
                tracing::error!("      {}", pubkey);
            }
            if count > MAX_SAMPLE_PUBKEYS {
                tracing::error!("      ... and {} more", count - MAX_SAMPLE_PUBKEYS);
            }
        }
    }

    Ok(())
}
