use carbon_core::{CarbonDeserialize, borsh};

use crate::types::ProcessStatus;

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x69b80569af700da9")]
pub struct CraftingProcess {
    pub version: u8,
    pub crafting_id: u64,
    pub authority: solana_pubkey::Pubkey,
    pub recipe: solana_pubkey::Pubkey,
    pub crafting_facility: solana_pubkey::Pubkey,
    pub inputs_checksum: [u8; 16],
    pub outputs_checksum: [u8; 16],
    pub quantity: u64,
    pub status: ProcessStatus,
    pub start_time: i64,
    pub end_time: i64,
    pub deny_permissionless_claiming: u8,
    pub use_local_time: u8,
    pub bump: u8,
}
