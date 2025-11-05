use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PointsLevel {
    pub level: u16,
    pub points: u64,
    pub token_vault: solana_pubkey::Pubkey,
    pub token_qty: u64,
}
