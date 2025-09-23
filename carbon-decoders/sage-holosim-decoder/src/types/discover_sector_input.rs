use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct DiscoverSectorInput {
    pub coordinates: [i64; 2],
    pub key_index: u16,
}
