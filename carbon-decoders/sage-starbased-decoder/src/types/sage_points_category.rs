use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SagePointsCategory {
    pub category: solana_pubkey::Pubkey,
    pub modifier: solana_pubkey::Pubkey,
    pub modifier_bump: u8,
}
