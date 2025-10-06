use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RegisterMineItemInput {
    pub name: Vec<u8>,
    pub resource_hardness: u16,
    pub key_index: u16,
}
