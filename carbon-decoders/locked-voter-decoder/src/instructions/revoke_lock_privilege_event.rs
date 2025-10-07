use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d350fb25099c64191")]
pub struct RevokeLockPrivilegeEvent {
    pub locker: solana_pubkey::Pubkey,
    pub program_id: solana_pubkey::Pubkey,
    pub timestamp: i64,
}
