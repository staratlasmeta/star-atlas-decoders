use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RegisterPlanetInput {
    pub name: Vec<u8>,
    pub size: u64,
    pub max_hp: u64,
    pub sub_coordinates: [i64; 2],
    pub planet_type: u8,
    pub position: u8,
    pub key_index: u16,
}
