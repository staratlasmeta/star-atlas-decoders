use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x80f5ee8ae230d83f")]
pub struct LockerWhitelistEntry {
    pub bump: u8,
    pub locker: solana_pubkey::Pubkey,
    pub program_id: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
}
