use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateShipFleetInput {
    pub ship_amount: u16,
    pub fleet_ship_info_index: u32,
}
