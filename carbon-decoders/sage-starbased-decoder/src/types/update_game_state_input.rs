use super::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateGameStateInput {
    pub fleet: Option<FleetInput>,
    pub misc: Option<MiscVariablesInput>,
    pub key_index: u16,
}
