use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d72d692efe5128c9f")]
pub struct CombatLootDropEvent {
    pub combat_id: [u8; 32],
    pub destroyed_fleet: solana_pubkey::Pubkey,
    pub destroyer_fleet: solana_pubkey::Pubkey,
    pub loot_location_x: i64,
    pub loot_location_y: i64,
    pub loot_account: solana_pubkey::Pubkey,
    pub loot_exclusivity_time: i64,
    pub timestamp: i64,
}
