use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct WarpToCoordinateInput {
    pub key_index: u16,
    pub to_sector: [i64; 2],
}
