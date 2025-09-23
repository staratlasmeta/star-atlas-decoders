use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AddShipToFleetInput {
    pub ship_amount: u8,
    pub ship_escrow_index: u32,
    pub fleet_ship_info_index: Option<u32>,
    pub key_index: u16,
}
