use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x03fc32a24c2e90d5")]
pub struct FeePayer {
    pub version: u8,
    pub rates: solana_pubkey::Pubkey,
    pub token_vault: solana_pubkey::Pubkey,
    pub payment_account: solana_pubkey::Pubkey,
    pub last_payer_value: u64,
}
