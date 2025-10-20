use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x90c32ec979c333fc")]
pub struct PointsStore {
    pub version: u8,
    pub point_category: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
    pub price: u64,
    pub signer_bump: u8,
}
