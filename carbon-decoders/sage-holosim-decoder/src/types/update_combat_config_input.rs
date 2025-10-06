use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateCombatConfigInput {
    pub key_index: u16,
    pub global_ceasefire: u8,
    pub loot_exclusivity_time: Option<u16>,
    pub starbase_upgrade_progress_continuation: Option<u32>,
    pub crew_respawn_time: Option<u16>,
    pub raw_ships_respawn_time: Option<u16>,
}
