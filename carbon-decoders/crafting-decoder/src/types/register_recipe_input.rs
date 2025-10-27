use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RegisterRecipeInput {
    pub duration: i64,
    pub min_duration: i64,
    pub namespace: [u8; 32],
    pub fee_amount: Option<u64>,
    pub usage_limit: Option<u64>,
    pub value: Option<u64>,
    pub key_index: u16,
}
