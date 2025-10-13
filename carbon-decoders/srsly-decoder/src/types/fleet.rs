

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct Fleet {
    pub version: u8,
    pub game_id: solana_pubkey::Pubkey,
    pub owner_profile: solana_pubkey::Pubkey,
    pub fleet_ships: solana_pubkey::Pubkey,
    pub sub_profile: solana_pubkey::Pubkey,
    pub sub_profile_invalidator: solana_pubkey::Pubkey,
}
