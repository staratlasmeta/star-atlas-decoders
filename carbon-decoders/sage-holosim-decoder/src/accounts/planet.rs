use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf21bec2adcd98480")]
pub struct Planet {
    pub version: u8,
    #[serde(with = "serde_big_array::BigArray")]
    pub name: [u8; 64],
    pub game_id: solana_pubkey::Pubkey,
    pub sector: [i64; 2],
    pub sub_coordinates: [i64; 2],
    pub planet_type: u8,
    pub position: u8,
    pub size: u64,
    pub max_hp: u64,
    pub current_health: u64,
    pub amount_mined: u64,
    pub num_resources: u8,
    pub num_miners: u64,
}
