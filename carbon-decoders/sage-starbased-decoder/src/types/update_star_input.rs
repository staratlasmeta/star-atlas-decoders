use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateStarInput {
    pub name: Option<Vec<u8>>,
    pub size: Option<u64>,
    pub star_type: Option<u8>,
    pub key_index: u16,
}
