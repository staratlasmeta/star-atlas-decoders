use super::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RegisterShipInput {
    pub name: Vec<u8>,
    pub size_class: SizeClass,
    pub stats: ShipStatsUnpacked,
    pub key_index: u16,
    pub is_active: bool,
}
