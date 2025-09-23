use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RegisterStarbaseInput {
    pub name: Vec<u8>,
    pub sub_coordinates: [i64; 2],
    pub starbase_level_index: u8,
    pub faction: u8,
    pub key_index: u16,
}
