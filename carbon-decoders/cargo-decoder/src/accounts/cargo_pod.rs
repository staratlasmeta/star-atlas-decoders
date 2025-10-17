use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa52176ebfcbcf45d")]
pub struct CargoPod {
    pub version: u8,
    pub stats_definition: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub open_token_accounts: u8,
    pub pod_seeds: [u8; 32],
    pub pod_bump: u8,
    pub seq_id: u16,
    pub unupdated_token_accounts: u8,
}
