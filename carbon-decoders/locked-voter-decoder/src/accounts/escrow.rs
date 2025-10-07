use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1fd57bbbba16da9b")]
pub struct Escrow {
    pub locker: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub bump: u8,
    pub tokens: solana_pubkey::Pubkey,
    pub amount: u64,
    pub escrow_started_at: i64,
    pub escrow_ends_at: i64,
    pub vote_delegate: solana_pubkey::Pubkey,
}
