use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RecipeIngredients {
    pub amount: u64,
    pub mint: solana_pubkey::Pubkey,
    pub key_index: u16,
}
