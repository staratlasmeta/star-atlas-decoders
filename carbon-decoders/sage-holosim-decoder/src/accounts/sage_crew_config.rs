use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf72a186da103f8a6")]
pub struct SageCrewConfig {
    pub version: u8,
    pub game_id: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub bump: u8,
}
