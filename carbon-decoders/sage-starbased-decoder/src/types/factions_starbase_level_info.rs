use super::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct FactionsStarbaseLevelInfo {
    pub mud: [StarbaseLevelInfo; 7],
    pub oni: [StarbaseLevelInfo; 7],
    pub ustur: [StarbaseLevelInfo; 7],
}
