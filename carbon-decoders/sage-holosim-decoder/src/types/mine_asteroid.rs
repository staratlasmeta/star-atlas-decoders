use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MineAsteroid {
    pub asteroid: solana_pubkey::Pubkey,
    pub resource: solana_pubkey::Pubkey,
    pub start: i64,
    pub end: i64,
    pub amount_mined: u64,
    pub last_update: i64,
}
