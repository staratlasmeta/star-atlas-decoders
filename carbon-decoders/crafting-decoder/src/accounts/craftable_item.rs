use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7cf63808685ff9fb")]
pub struct CraftableItem {
    pub version: u8,
    pub domain: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub namespace: [u8; 32],
    pub bump: u8,
}
