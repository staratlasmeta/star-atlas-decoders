use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3c8885918111836d")]
pub struct CargoStatsDefinition {
    pub version: u8,
    pub authority: solana_pubkey::Pubkey,
    pub default_cargo_type: solana_pubkey::Pubkey,
    pub stats_count: u16,
    pub seq_id: u16,
}
