use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateMineItemInput {
    pub name: Option<Vec<u8>>,
    pub resource_hardness: Option<u16>,
    pub key_index: u16,
}
