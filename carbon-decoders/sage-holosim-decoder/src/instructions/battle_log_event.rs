use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d4f06f081f297f573")]
pub struct BattleLogEvent {
    pub combat_id: [u8; 32],
    pub attacker_fleet: solana_pubkey::Pubkey,
    pub defender_fleet: solana_pubkey::Pubkey,
    pub sector: [i64; 2],
    pub attacker_pre_hp: u32,
    pub attacker_pre_sp: u32,
    pub defender_pre_hp: u32,
    pub defender_pre_sp: u32,
    pub damage_to_attacker_hp: u32,
    pub damage_to_attacker_sp: u32,
    pub damage_to_defender_hp: u32,
    pub damage_to_defender_sp: u32,
    pub attacker_post_hp: u32,
    pub attacker_post_sp: u32,
    pub defender_post_hp: u32,
    pub defender_post_sp: u32,
    pub attacker_xp_gained: u32,
    pub defender_xp_gained: u32,
}
