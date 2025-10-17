use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc7998eec631a18ce")]
pub struct RecipeCategory {
    pub version: u8,
    pub domain: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub recipe_count: u32,
    pub namespace: [u8; 32],
}
