use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d951fcf6aac9b416e")]
pub struct NewLockerEvent {
    pub governor: solana_pubkey::Pubkey,
    pub locker: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub params: LockerParams,
}
