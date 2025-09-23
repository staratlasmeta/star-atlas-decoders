use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct WarpLaneInput {
    pub key_index: u16,
    pub to_sector_index: u16,
    pub from_sector_index: u16,
}
