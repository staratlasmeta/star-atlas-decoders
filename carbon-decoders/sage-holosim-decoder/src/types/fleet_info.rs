use super::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct FleetInfo {
    pub starbase_levels: FactionsStarbaseLevelInfo,
    pub upkeep: StarbaseUpkeepLevels,
    pub max_fleet_size: u32,
}
