use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateRedemptionEpochInput {
    pub total_tokens: u64,
    pub epoch_index: u16,
    pub key_index: u16,
}
