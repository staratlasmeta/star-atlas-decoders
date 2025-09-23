use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd683cfd0ca94a230")]
pub struct Star {
    pub version: u8,
    #[serde(with = "serde_big_array::BigArray")]
    pub name: [u8; 64],
    pub game_id: solana_pubkey::Pubkey,
    pub sector: [i64; 2],
    pub size: u64,
    pub sub_coordinates: [i64; 2],
    pub star_type: u8,
}
