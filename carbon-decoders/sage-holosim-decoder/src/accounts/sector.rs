use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x417517525085f7e9")]
pub struct Sector {
    pub version: u8,
    pub game_id: solana_pubkey::Pubkey,
    pub coordinates: [i64; 2],
    pub discoverer: solana_pubkey::Pubkey,
    #[serde(with = "serde_big_array::BigArray")]
    pub name: [u8; 64],
    pub num_stars: u16,
    pub num_planets: u16,
    pub num_moons: u16,
    pub num_asteroid_belts: u16,
    pub last_scan_time: i64,
    pub last_scan_chance: u32,
    pub bump: u8,
    pub num_connections: u16,
}
