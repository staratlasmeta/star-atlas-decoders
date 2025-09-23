use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d549d1723c97c0924")]
pub struct CombatInitiatedEvent {
    pub attacker_fleet: solana_pubkey::Pubkey,
    pub defender_fleet: solana_pubkey::Pubkey,
    pub sector_key: solana_pubkey::Pubkey,
    pub combat_id: [u8; 32],
    pub timestamp: i64,
    pub attacker_faction: u8,
    pub defender_faction: u8,
}
