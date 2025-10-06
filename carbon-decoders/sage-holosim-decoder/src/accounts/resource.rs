use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0aa002012acf33d4")]
pub struct Resource {
    pub version: u8,
    pub game_id: solana_pubkey::Pubkey,
    pub location: solana_pubkey::Pubkey,
    pub mine_item: solana_pubkey::Pubkey,
    pub location_type: u8,
    pub system_richness: u16,
    pub amount_mined: u64,
    pub num_miners: u64,
    pub bump: u8,
}
