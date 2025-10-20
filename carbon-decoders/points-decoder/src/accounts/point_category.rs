use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf80754ca32688f22")]
pub struct PointCategory {
    pub version: u8,
    pub profile: solana_pubkey::Pubkey,
    pub token_required: u8,
    pub token_mint: solana_pubkey::Pubkey,
    pub token_qty: u64,
    pub transfer_tokens_to_vault: u8,
    pub token_vault: solana_pubkey::Pubkey,
    pub point_limit: u64,
    pub is_spendable: u8,
    pub post_levels_upgrade_threshold: u64,
}
