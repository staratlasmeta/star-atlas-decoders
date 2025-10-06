use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MiscVariables {
    pub warp_lane_fuel_cost_reduction: i16,
    pub respawn_fee: u64,
    pub upkeep_mining_emissions_penalty: i16,
}
