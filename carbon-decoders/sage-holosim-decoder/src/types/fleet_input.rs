use super::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct FleetInput {
    pub starbase_level_info_array: Option<Vec<StarbaseLevelInfoArrayInput>>,
    pub upkeep_info_array: Option<Vec<StarbaseUpkeepInfoArrayInput>>,
    pub max_fleet_size: Option<u32>,
}
