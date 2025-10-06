use super::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct StarbaseUpkeepLevels {
    pub level0: StarbaseUpkeepInfo,
    pub level1: StarbaseUpkeepInfo,
    pub level2: StarbaseUpkeepInfo,
    pub level3: StarbaseUpkeepInfo,
    pub level4: StarbaseUpkeepInfo,
    pub level5: StarbaseUpkeepInfo,
    pub level6: StarbaseUpkeepInfo,
}
