use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4037d413d79c1642")]
pub struct MineItem {
    pub version: u8,
    pub game_id: solana_pubkey::Pubkey,
    #[serde(with = "serde_big_array::BigArray")]
    pub name: [u8; 64],
    pub mint: solana_pubkey::Pubkey,
    pub resource_hardness: u16,
    pub num_resource_accounts: u64,
    pub bump: u8,
}
