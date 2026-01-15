use anyhow::{Context, Result};
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
};
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{account::Account, pubkey::Pubkey};

/// Initialize RPC client and verify connection
pub fn init_rpc_client(rpc_url: &str) -> Result<RpcClient> {
    tracing::info!("Connecting to RPC...");

    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let version = client
        .get_version()
        .context("Failed to connect to RPC - verify RPC_URL is correct")?;

    tracing::info!("Connected to Solana RPC version: {}", version.solana_core);

    Ok(client)
}

/// Get all program accounts with proper configuration
pub fn get_program_accounts(
    client: &RpcClient,
    program_id: &Pubkey,
) -> Result<Vec<(Pubkey, Account)>> {
    tracing::info!("Fetching all accounts for program: {}", program_id);

    let config = RpcProgramAccountsConfig {
        filters: None,
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            data_slice: None,
            commitment: Some(CommitmentConfig::confirmed()),
            min_context_slot: None,
        },
        with_context: None,
        sort_results: None,
    };

    let ui_accounts = client
        .get_program_ui_accounts_with_config(program_id, config)
        .context("Failed to fetch program accounts")?;

    let accounts: Vec<(Pubkey, Account)> = ui_accounts
        .into_iter()
        .filter_map(|(address, ui_account)| {
            let account = ui_account.decode()?;
            Some((address, account))
        })
        .collect();

    tracing::info!("Fetched {} accounts", accounts.len());

    Ok(accounts)
}
