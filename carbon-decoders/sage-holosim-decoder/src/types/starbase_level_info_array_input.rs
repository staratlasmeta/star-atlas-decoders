use super::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct StarbaseLevelInfoArrayInput {
    pub level: u8,
    pub faction: u8,
    pub hp: u64,
    pub sp: u64,
    pub sector_ring_available: SectorRing,
    pub warp_lane_movement_fee: u64,
    pub repair_fee: u64,
    pub repair_efficiency: u32,
    pub shield_recharge_rate: u32,
    pub shield_break_delay: u32,
}
