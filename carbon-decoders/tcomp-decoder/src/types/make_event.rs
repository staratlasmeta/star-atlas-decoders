use super::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MakeEvent {
    pub maker: solana_pubkey::Pubkey,
    pub bid_id: Option<solana_pubkey::Pubkey>,
    pub target: Target,
    pub target_id: solana_pubkey::Pubkey,
    pub field: Option<Field>,
    pub field_id: Option<solana_pubkey::Pubkey>,
    pub amount: u64,
    pub quantity: u32,
    pub currency: Option<solana_pubkey::Pubkey>,
    pub expiry: i64,
    pub private_taker: Option<solana_pubkey::Pubkey>,
    pub asset_id: Option<solana_pubkey::Pubkey>,
}
