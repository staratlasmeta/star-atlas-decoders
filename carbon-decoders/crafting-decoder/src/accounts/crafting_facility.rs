use carbon_core::{CarbonDeserialize, borsh};

use crate::types::LocationType;

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3a4923115cf7311e")]
pub struct CraftingFacility {
    pub version: u8,
    pub domain: solana_pubkey::Pubkey,
    pub location: solana_pubkey::Pubkey,
    pub location_type: LocationType,
    pub max_concurrent_processes: u32,
    pub num_concurrent_processes: u32,
    pub efficiency: u32,
    pub num_recipe_categories: u32,
}
