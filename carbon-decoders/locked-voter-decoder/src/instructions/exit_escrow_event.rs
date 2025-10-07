use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dda5b44bd6698d4a6")]
pub struct ExitEscrowEvent {
    pub escrow_owner: solana_pubkey::Pubkey,
    pub locker: solana_pubkey::Pubkey,
    pub timestamp: i64,
    pub locker_supply: u64,
    pub released_amount: u64,
}
