use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RegisterPackTiersInput {
    pub key_index: u16,
    pub seed_pubkey: solana_pubkey::Pubkey,
    pub pack_tier: u8,
    pub common: u32,
    pub uncommon: u32,
    pub rare: u32,
    pub epic: u32,
    pub legendary: u32,
    pub anomaly: u32,
}
