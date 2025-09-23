use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RegisterCombatConfigInput {
    pub loot_exclusivity_time: u16,
    pub starbase_upgrade_progress_continuation: u32,
    pub crew_respawn_time: u16,
    pub raw_ships_respawn_time: u16,
    pub key_index: u16,
}
