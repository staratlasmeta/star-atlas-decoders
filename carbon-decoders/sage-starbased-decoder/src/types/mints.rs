use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Mints {
    pub atlas: solana_pubkey::Pubkey,
    pub polis: solana_pubkey::Pubkey,
    pub ammo: solana_pubkey::Pubkey,
    pub food: solana_pubkey::Pubkey,
    pub fuel: solana_pubkey::Pubkey,
    pub repair_kit: solana_pubkey::Pubkey,
}
