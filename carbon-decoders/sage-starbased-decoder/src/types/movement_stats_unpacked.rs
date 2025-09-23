use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MovementStatsUnpacked {
    pub subwarp_speed: u32,
    pub warp_speed: u32,
    pub max_warp_distance: u16,
    pub warp_cool_down: u16,
    pub subwarp_fuel_consumption_rate: u32,
    pub warp_fuel_consumption_rate: u32,
    pub planet_exit_fuel_amount: u32,
}
