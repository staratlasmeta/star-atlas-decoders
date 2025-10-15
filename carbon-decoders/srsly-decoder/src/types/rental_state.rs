use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RentalState {
    pub version: u8,
    pub borrower: solana_pubkey::Pubkey,
    pub thread: solana_pubkey::Pubkey,
    pub contract: solana_pubkey::Pubkey,
    pub owner_token_account: solana_pubkey::Pubkey,
    pub rate: i64, // FIXME: temporary hack for rate (f64)
    pub start_time: i64,
    pub end_time: i64,
    pub cancelled: bool,
    pub bump: u8,
}
