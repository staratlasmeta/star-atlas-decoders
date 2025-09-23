use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CombatStatsUnpacked {
    pub ap: u32,
    pub hp: u32,
    pub sp: u32,
    pub ammo_consumption_rate: u32,
    pub ap_regen_rate: u32,
    pub shield_recharge_rate: u32,
    pub repair_rate: u32,
    pub repair_ability: u32,
    pub repair_efficiency: u32,
    pub loot_rate: u32,
    pub shield_break_delay: u16,
    pub warp_spool_duration: u16,
}
