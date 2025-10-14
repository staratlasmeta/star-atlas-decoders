use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdatePackTiersInput {
    pub key_index: u16,
    pub common: Option<u32>,
    pub uncommon: Option<u32>,
    pub rare: Option<u32>,
    pub epic: Option<u32>,
    pub legendary: Option<u32>,
    pub anomaly: Option<u32>,
}
