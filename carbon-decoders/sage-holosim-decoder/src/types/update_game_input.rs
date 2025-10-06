use super::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateGameInput {
    pub cargo: u8,
    pub crafting: u8,
    pub mints: u8,
    pub vaults: u8,
    pub points: u8,
    pub risk_zones: Option<RiskZonesDataUnpacked>,
    pub key_index: u16,
}
