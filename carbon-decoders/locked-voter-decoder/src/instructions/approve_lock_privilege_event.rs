use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1df0e0893d8fc8e16e")]
pub struct ApproveLockPrivilegeEvent {
    pub locker: solana_pubkey::Pubkey,
    pub program_id: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub timestamp: i64,
}
