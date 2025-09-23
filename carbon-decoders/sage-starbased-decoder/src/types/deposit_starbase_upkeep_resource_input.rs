use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct DepositStarbaseUpkeepResourceInput {
    pub points_program_permissions_key_index: u16,
    pub sage_permissions_key_index: u16,
    pub resource_type: u8,
    pub resource_index: u16,
    pub amount: u64,
    pub epoch_index: u16,
}
