use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ContributeToRedemptionInput {
    pub points: u64,
    pub epoch_index: u16,
    pub key_index: u16,
}
