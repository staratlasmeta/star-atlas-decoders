use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RegisterRecipeCategoryInput {
    pub namespace: [u8; 32],
    pub key_index: u16,
}
