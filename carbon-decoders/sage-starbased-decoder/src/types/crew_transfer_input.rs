use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CrewTransferInput {
    pub data_hash: [u8; 32],
    pub proof_count: u8,
    pub leaf_index: u32,
}
