use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct DisbandedFleetToEscrowInput {
    pub ship_amount: u16,
    pub ship_escrow_index: Option<u32>,
    pub fleet_ship_info_index: u32,
    pub key_index: u16,
}
