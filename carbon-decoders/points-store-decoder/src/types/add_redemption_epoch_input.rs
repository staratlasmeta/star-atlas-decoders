use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AddRedemptionEpochInput {
    pub total_tokens: u64,
    pub day_index: i64,
    pub key_index: u16,
}
