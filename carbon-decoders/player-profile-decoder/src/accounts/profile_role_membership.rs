use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2d02566f6e790501")]
pub struct ProfileRoleMembership {
    pub version: u8,
    pub profile: solana_pubkey::Pubkey,
    pub member: solana_pubkey::Pubkey,
    pub bump: u8,
}
