use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbbf8b502b7a542af")]
pub struct FeeReduction {
    pub account: solana_pubkey::Pubkey,
    pub bump: u8,
    pub discount: u64,
}
