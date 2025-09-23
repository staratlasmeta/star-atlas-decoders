use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ShipSizes {
    pub xx_small: u8,
    pub x_small: u8,
    pub small: u8,
    pub medium: u8,
    pub large: u8,
    pub capital: u8,
    pub commander: u8,
    pub titan: u8,
}
