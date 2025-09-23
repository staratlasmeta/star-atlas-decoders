use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7229f5e8183aea9e")]
pub struct Ship {
    pub version: u8,
    pub game_id: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    #[serde(with = "serde_big_array::BigArray")]
    pub name: [u8; 64],
    pub size_class: u8,
    pub stats: ShipStats,
    pub update_id: u64,
    pub max_update_id: u64,
    pub next: OptionalNonSystemPubkey,
}
