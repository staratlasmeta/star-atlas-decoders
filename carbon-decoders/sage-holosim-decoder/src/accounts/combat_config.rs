use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf5d3483f2c8276c1")]
pub struct CombatConfig {
    pub version: u8,
    pub game_id: solana_pubkey::Pubkey,
    pub global_ceasefire: u8,
    pub loot_exclusivity_time: u16,
    pub starbase_upgrade_progress_continuation: u32,
    pub crew_respawn_time: u16,
    pub raw_ships_respawn_time: u16,
}
