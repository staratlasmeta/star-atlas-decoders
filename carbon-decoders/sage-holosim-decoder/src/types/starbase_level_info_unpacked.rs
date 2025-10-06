use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct StarbaseLevelInfoUnpacked {
    pub recipe_for_upgrade: solana_pubkey::Pubkey,
    pub recipe_category_for_level: solana_pubkey::Pubkey,
    pub hp: u64,
    pub sp: u64,
    pub sector_ring_available: u8,
    pub warp_lane_movement_fee: u64,
    pub repair_fee: u64,
    pub repair_efficiency: u32,
    pub shield_recharge_rate: u32,
    pub shield_break_delay: u32,
}
