use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1def18d1ead28f07ca")]
pub struct LockerSetParamsEvent {
    pub locker: solana_pubkey::Pubkey,
    pub prev_params: LockerParams,
    pub params: LockerParams,
}
