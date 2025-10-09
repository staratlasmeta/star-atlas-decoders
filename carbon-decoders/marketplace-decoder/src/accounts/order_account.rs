use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4f43709bd60e2037")]
pub struct OrderAccount {
    pub order_initializer_pubkey: solana_pubkey::Pubkey,
    pub currency_mint: solana_pubkey::Pubkey,
    pub asset_mint: solana_pubkey::Pubkey,
    pub initializer_currency_token_account: solana_pubkey::Pubkey,
    pub initializer_asset_token_account: solana_pubkey::Pubkey,
    pub order_side: OrderSide,
    pub price: u64,
    pub order_origination_qty: u64,
    pub order_remaining_qty: u64,
    pub created_at_timestamp: i64,
}
