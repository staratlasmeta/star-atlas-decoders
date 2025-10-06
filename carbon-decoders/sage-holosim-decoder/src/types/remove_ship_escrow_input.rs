use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RemoveShipEscrowInput {
    pub ship_amount: u64,
    pub permission_key_index: u16,
    pub ship_escrow_index: u32,
}
