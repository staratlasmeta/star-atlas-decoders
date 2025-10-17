use super::super::types::*;

use crate::types::RecipeStatus;
use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0aa29c6438c1cd4d")]
pub struct Recipe {
    pub version: u8,
    pub domain: solana_pubkey::Pubkey,
    pub category: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub duration: i64,
    pub min_duration: i64,
    pub namespace: [u8; 32],
    pub status: RecipeStatus,
    pub fee_amount: u64,
    pub fee_recipient: OptionalNonSystemPubkey,
    pub usage_count: u64,
    pub usage_limit: u64,
    pub value: u64,
    pub consumables_count: u8,
    pub non_consumables_count: u8,
    pub outputs_count: u8,
    pub total_count: u16,
}
