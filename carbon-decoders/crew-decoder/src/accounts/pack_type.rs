use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x39cb3822d4419003")]
pub struct PackType {
    pub version: u8,
    pub bump: u8,
    pub crew_config: solana_pubkey::Pubkey,
    pub pack_tiers: solana_pubkey::Pubkey,
    pub faction: u8,
}
