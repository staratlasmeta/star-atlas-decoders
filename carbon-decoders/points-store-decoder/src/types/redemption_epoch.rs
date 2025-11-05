use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RedemptionEpoch {
    pub total_points: u64,
    pub redeemed_points: u64,
    pub total_tokens: u64,
    pub redeemed_tokens: u64,
    pub day_index: i64,
}
