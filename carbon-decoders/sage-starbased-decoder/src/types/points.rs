use super::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Points {
    pub lp_category: SagePointsCategory,
    pub council_rank_xp_category: SagePointsCategory,
    pub pilot_xp_category: SagePointsCategory,
    pub data_running_xp_category: SagePointsCategory,
    pub mining_xp_category: SagePointsCategory,
    pub crafting_xp_category: SagePointsCategory,
}
