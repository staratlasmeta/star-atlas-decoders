use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CargoStatsUnpacked {
    pub cargo_capacity: u32,
    pub fuel_capacity: u32,
    pub ammo_capacity: u32,
    pub ammo_consumption_rate: u32,
    pub food_consumption_rate: u32,
    pub mining_rate: u32,
    pub upgrade_rate: u32,
    pub cargo_transfer_rate: u32,
    pub tractor_beam_gather_rate: u32,
}
