use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MiscVariablesInput {
    pub warp_lane_fuel_cost_reduction: Option<i16>,
    pub upkeep_mining_emissions_penalty: Option<i16>,
    pub respawn_fee: Option<u64>,
}
