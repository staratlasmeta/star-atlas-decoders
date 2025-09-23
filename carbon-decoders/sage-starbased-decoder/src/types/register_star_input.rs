use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RegisterStarInput {
    pub name: Vec<u8>,
    pub size: u64,
    pub sub_coordinates: [i64; 2],
    pub star_type: u8,
    pub key_index: u16,
}
