use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xccb61de7dc1d3402")]
pub struct Starbase {
    pub version: u8,
    pub game_id: solana_pubkey::Pubkey,
    pub sector: [i64; 2],
    pub crafting_facility: solana_pubkey::Pubkey,
    pub upgrade_facility: solana_pubkey::Pubkey,
    #[serde(with = "serde_big_array::BigArray")]
    pub name: [u8; 64],
    pub sub_coordinates: [i64; 2],
    pub faction: u8,
    pub bump: u8,
    pub seq_id: u16,
    pub state: u8,
    pub level: u8,
    pub hp: u64,
    pub sp: u64,
    pub sector_ring_available: u8,
    pub upgrade_state: u8,
    pub upgrade_ingredients_checksum: [u8; 16],
    pub num_upgrade_ingredients: u8,
    pub upkeep_ammo_balance: u64,
    pub upkeep_ammo_last_update: i64,
    pub upkeep_ammo_global_last_update: i64,
    pub upkeep_food_balance: u64,
    pub upkeep_food_last_update: i64,
    pub upkeep_food_global_last_update: i64,
    pub upkeep_toolkit_balance: u64,
    pub upkeep_toolkit_last_update: i64,
    pub upkeep_toolkit_global_last_update: i64,
    pub built_destroyed_timestamp: i64,
}
