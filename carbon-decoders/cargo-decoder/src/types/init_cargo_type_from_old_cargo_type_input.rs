use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitCargoTypeFromOldCargoTypeInput {
    pub key_index: u16,
    pub new_values: Option<Vec<u64>>,
}
