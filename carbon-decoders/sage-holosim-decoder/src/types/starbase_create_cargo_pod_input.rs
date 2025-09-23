use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct StarbaseCreateCargoPodInput {
    pub pod_seeds: [u8; 32],
    pub key_index: u16,
}
