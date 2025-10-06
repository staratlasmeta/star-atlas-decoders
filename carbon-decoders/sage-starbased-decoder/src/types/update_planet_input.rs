use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdatePlanetInput {
    pub name: Option<Vec<u8>>,
    pub size: Option<u64>,
    pub max_hp: Option<u64>,
    pub sub_coordinates: Option<[i64; 2]>,
    pub key_index: u16,
}
