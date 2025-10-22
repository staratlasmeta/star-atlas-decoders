use carbon_core::{CarbonDeserialize, borsh};

use crate::types::Faction;

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0e9577f391f04fe3")]
pub struct ProfileFactionAccount {
    pub version: u8,
    pub profile: solana_pubkey::Pubkey,
    pub faction: Faction,
    pub bump: u8,
}
