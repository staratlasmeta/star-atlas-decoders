use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xad01562f1bcc92b9")]
pub struct RedemptionConfig {
    pub version: u8,
    pub point_category: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub faction: u8,
    pub bank: solana_pubkey::Pubkey,
    pub signer_bump: u8,
    pub allow_only_current_epoch: u8,
}
