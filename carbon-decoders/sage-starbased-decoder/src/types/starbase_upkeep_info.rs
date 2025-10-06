use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct StarbaseUpkeepInfo {
    pub ammo_reserve: u64,
    pub ammo_depletion_rate: u32,
    pub food_reserve: u64,
    pub food_depletion_rate: u32,
    pub toolkit_reserve: u64,
    pub toolkit_depletion_rate: u32,
}
