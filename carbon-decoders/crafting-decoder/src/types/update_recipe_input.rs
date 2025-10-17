use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateRecipeInput {
    pub duration: Option<i64>,
    pub min_duration: Option<i64>,
    pub fee_amount: Option<u64>,
    pub status: Option<u8>,
    pub usage_limit: Option<u64>,
    pub value: Option<u64>,
    pub key_index: u16,
}
