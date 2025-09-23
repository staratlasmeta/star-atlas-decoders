use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x905ed0acf8638678")]
pub struct GameState {
    pub version: u8,
    pub update_id: u64,
    pub game_id: solana_pubkey::Pubkey,
    pub fleet: FleetInfo,
    pub misc: MiscVariables,
    pub bump: u8,
}
