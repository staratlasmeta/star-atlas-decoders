use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3c72f48610a63395")]
pub struct RegisteredCurrency {
    pub token_mint: solana_pubkey::Pubkey,
    pub sa_currency_vault: solana_pubkey::Pubkey,
    pub royalty: u64,
    pub bump: u8,
    pub royalty_tiers: Vec<RoyaltyTier>,
}
