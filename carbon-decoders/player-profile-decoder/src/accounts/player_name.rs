use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6609f16260c4d4a1")]
pub struct PlayerName {
    pub version: u8,
    pub profile: solana_pubkey::Pubkey,
    pub bump: u8,
}
