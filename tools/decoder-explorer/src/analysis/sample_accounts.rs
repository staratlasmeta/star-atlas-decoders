use super::{AccountTypeIdentifier, AnalysisContext};
use anyhow::Result;
use carbon_core::account::AccountDecoder;

const DEFAULT_SAMPLE_SIZE: usize = 5;

pub fn analyze<'a, D, T>(ctx: &AnalysisContext<'a>, decoder: &D) -> Result<()>
where
    D: AccountDecoder<'a, AccountType = T>,
    T: AccountTypeIdentifier + serde::Serialize,
{
    tracing::info!("\n=== Sample Accounts ===");

    let sample_size = DEFAULT_SAMPLE_SIZE.min(ctx.total_accounts());
    tracing::info!("Showing first {} accounts:", sample_size);

    for (i, (pubkey, account)) in ctx.accounts.iter().take(sample_size).enumerate() {
        tracing::info!("\n--- Account {} ---", i + 1);
        tracing::info!("Pubkey: {}", pubkey);
        tracing::info!("Owner: {}", account.owner);
        tracing::info!("Lamports: {}", account.lamports);
        tracing::info!("Data length: {} bytes", account.data.len());
        tracing::info!("Executable: {}", account.executable);
        tracing::info!("Rent epoch: {}", account.rent_epoch);

        match decoder.decode_account(account) {
            Some(decoded) => {
                let type_name = decoded.data.type_name();
                tracing::info!("Account type: {}", type_name);

                // Serialize to JSON for inspection
                match serde_json::to_string_pretty(&decoded.data) {
                    Ok(json) => {
                        tracing::debug!("Decoded data:\n{}", json);
                    }
                    Err(e) => {
                        tracing::warn!("Failed to serialize decoded data: {}", e);
                    }
                }
            }
            None => {
                tracing::warn!("Failed to decode account");

                // Show hex preview of first 64 bytes
                let preview_len = account.data.len().min(64);
                let hex_preview: String = account.data[..preview_len]
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<_>>()
                    .join(" ");

                tracing::info!("Raw data (first {} bytes): {}", preview_len, hex_preview);
            }
        }
    }

    Ok(())
}
