use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xddb930074bc426db")]
pub struct PlayerCrewRecord {
    pub version: u8,
    pub player_profile: solana_pubkey::Pubkey,
    pub crew_config: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub count: u32,
    pub bump: u8,
}
