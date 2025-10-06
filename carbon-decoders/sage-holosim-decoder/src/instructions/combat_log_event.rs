// use super::super::types::*;

// use carbon_core::{CarbonDeserialize, borsh};

// #[derive(
//     CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
// )]
// #[carbon(discriminator = "0xe445a52e51cb9a1de04240179e2dca75")]
// pub struct CombatLogEvent {
//     pub combat_type: u8,
//     pub combat_id: [u8; 32],
//     pub timestamp: i64,
//     pub slot: u64,
//     pub attacker: CombatParticipant,
//     pub defender: CombatParticipant,
//     pub target_starbase_key: Option<solana_pubkey::Pubkey>,
//     pub target_starbase_faction: Option<u8>,
//     pub starbase_pre_hp: Option<u32>,
//     pub starbase_post_hp: Option<u32>,
//     pub starbase_max_hp: Option<u32>,
//     pub attack_damage_dealt: u32,
//     pub defense_damage_dealt: u32,
//     pub attacker_destroyed: bool,
//     pub defender_destroyed: bool,
//     pub starbase_destroyed: bool,
//     pub starbase_captured: bool,
//     pub attacker_xp_gained: u32,
//     pub defender_xp_gained: u32,
//     pub attacker_trophies_gained: u16,
//     pub defender_trophies_gained: u16,
//     pub game_version: [u8; 16],
//     pub asteroid_key: Option<solana_pubkey::Pubkey>,
//     pub was_retaliation: bool,
//     pub battle_duration: u16,
//     pub combat_random_seed: u64,
// }
