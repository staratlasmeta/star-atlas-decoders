use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4af60671f9e44ba9")]
pub struct Locker {
    pub base: solana_pubkey::Pubkey,
    pub bump: u8,
    pub token_mint: solana_pubkey::Pubkey,
    pub locked_supply: u64,
    pub governor: solana_pubkey::Pubkey,
    pub params: LockerParams,
}
