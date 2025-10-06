use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d9a27fa9508b14407")]
pub struct StarbaseCombatEvent {
    pub attacker_fleet: solana_pubkey::Pubkey,
    pub target_starbase: solana_pubkey::Pubkey,
    pub sector_key: solana_pubkey::Pubkey,
    pub combat_id: [u8; 32],
    pub timestamp: i64,
    pub attacker_faction: u8,
    pub starbase_faction: u8,
    pub damage_dealt: u32,
    pub starbase_destroyed: bool,
    pub starbase_captured: bool,
    pub starbase_hp_before: u32,
    pub starbase_hp_after: u32,
}
