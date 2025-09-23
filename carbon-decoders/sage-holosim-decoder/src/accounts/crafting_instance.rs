use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5aba9bd05dba70bf")]
pub struct CraftingInstance {
    pub version: u8,
    pub seq_id: u16,
    pub authority: solana_pubkey::Pubkey,
    pub crafting_process: solana_pubkey::Pubkey,
    pub instance_type: u8,
    pub num_crew: u64,
    pub bump: u8,
}
