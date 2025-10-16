use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2edbc518e9f9fd9a")]
pub struct Role {
    pub version: u8,
    pub profile: solana_pubkey::Pubkey,
    pub authorizer: solana_pubkey::Pubkey,
    pub role_seq_id: u64,
    pub accepting_new_members: u8,
    pub bump: u8,
}
