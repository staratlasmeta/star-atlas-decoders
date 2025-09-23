use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ShipCountsUnpacked {
    pub total: u32,
    pub updated: u32,
    pub xx_small: u16,
    pub x_small: u16,
    pub small: u16,
    pub medium: u16,
    pub large: u16,
    pub capital: u16,
    pub commander: u16,
    pub titan: u16,
}
