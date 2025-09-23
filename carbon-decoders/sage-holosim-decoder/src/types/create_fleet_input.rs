use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CreateFleetInput {
    pub ship_amount: u8,
    pub fleet_label: [u8; 32],
    pub ship_escrow_index: u32,
    pub cargo_hold_seeds: [u8; 32],
    pub fuel_tank_seeds: [u8; 32],
    pub ammo_bank_seeds: [u8; 32],
    pub key_index: u16,
}
