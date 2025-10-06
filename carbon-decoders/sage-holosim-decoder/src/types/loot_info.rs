use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LootInfo {
    pub exclusivity_unlock_time: i64,
    pub destroyer: solana_pubkey::Pubkey,
    pub loot: solana_pubkey::Pubkey,
}
