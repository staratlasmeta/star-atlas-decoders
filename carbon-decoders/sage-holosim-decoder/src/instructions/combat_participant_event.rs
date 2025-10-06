use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1def8991d1b6581bd5")]
pub struct CombatParticipantEvent {
    pub fleet_key: solana_pubkey::Pubkey,
    pub fleet_label: Vec<u8>,
    pub fleet_state: u8,
    pub owner_profile: solana_pubkey::Pubkey,
    pub player_key: solana_pubkey::Pubkey,
    pub faction: u8,
    pub public_key: solana_pubkey::Pubkey,
    pub pre_hp: u32,
    pub pre_sp: u32,
    pub pre_ap: u32,
    pub post_hp: u32,
    pub post_sp: u32,
    pub post_ap: u32,
    pub total_attack_power: u32,
    pub total_defense_power: u32,
    pub agility: u32,
    pub ship_class_counts: [u32; 8],
    pub position_x: i64,
    pub position_y: i64,
    pub sector_key: solana_pubkey::Pubkey,
    pub ammo_count: u64,
    pub fuel_count: u64,
    pub combat_xp: u64,
    pub council_rank: u16,
}
