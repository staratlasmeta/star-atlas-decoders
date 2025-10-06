use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RegisterResourceInput {
    pub location_type: u8,
    pub system_richness: u16,
    pub key_index: u16,
}
