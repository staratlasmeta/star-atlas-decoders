use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1b5aa67d4a647912")]
pub struct Game {
    pub version: u8,
    pub update_id: u64,
    pub profile: solana_pubkey::Pubkey,
    pub game_state: solana_pubkey::Pubkey,
    pub points: Points,
    pub cargo: Cargo,
    pub crafting: Crafting,
    pub mints: Mints,
    pub vaults: Vaults,
    pub risk_zones: RiskZonesData,
}
