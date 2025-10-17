use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9866bce6c80ea4e0")]
pub struct CargoType {
    pub version: u8,
    pub stats_definition: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub bump: u8,
    pub stats_count: u16,
    pub seq_id: u16,
}
