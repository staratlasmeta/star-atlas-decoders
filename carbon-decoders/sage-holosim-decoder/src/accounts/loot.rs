use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x97e1cfe473d2409f")]
pub struct Loot {
    pub version: u8,
    pub sector: [i64; 2],
    pub game_id: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub items: [LootInfo; 2],
}
