use super::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ShipStats {
    pub movement_stats: MovementStats,
    pub cargo_stats: CargoStats,
    pub combat_stats: CombatStats,
    pub misc_stats: MiscStats,
}
