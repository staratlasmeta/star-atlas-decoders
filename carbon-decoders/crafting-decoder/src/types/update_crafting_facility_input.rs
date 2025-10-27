use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateCraftingFacilityInput {
    pub efficiency: Option<u32>,
    pub max_concurrent_processes: Option<u32>,
    pub key_index: u16,
}
