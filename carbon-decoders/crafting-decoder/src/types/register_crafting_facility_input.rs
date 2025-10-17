use super::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RegisterCraftingFacilityInput {
    pub location_type: LocationType,
    pub efficiency: u32,
    pub max_concurrent_processes: u32,
    pub key_index: u16,
}
