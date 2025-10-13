

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct RedeemCrewPacksInput {
    pub quantity: u32,
    pub seed_pubkey: solana_pubkey::Pubkey,
    pub server_hash: [u8; 32],
    pub key_index: u16,
    pub sage_profile: Option<solana_pubkey::Pubkey>,
}
