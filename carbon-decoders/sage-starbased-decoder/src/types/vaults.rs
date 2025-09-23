use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Vaults {
    pub atlas: solana_pubkey::Pubkey,
    pub polis: solana_pubkey::Pubkey,
}
