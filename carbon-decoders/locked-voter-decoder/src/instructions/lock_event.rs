use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d4c2506ba0e2afd0f")]
pub struct LockEvent {
    pub locker: solana_pubkey::Pubkey,
    pub escrow_owner: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub amount: u64,
    pub locker_supply: u64,
    pub duration: i64,
    pub prev_escrow_ends_at: i64,
    pub next_escrow_ends_at: i64,
    pub next_escrow_started_at: i64,
}
