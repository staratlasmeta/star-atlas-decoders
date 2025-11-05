use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AddPointCategoryLevelInput {
    pub level: u16,
    pub points: u64,
    pub token_qty: Option<u64>,
    pub license: u8,
    pub key_index: u16,
}
