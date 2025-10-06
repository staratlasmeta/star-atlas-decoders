use super::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateProgressionConfigInput {
    pub key_index: u16,
    pub daily_lp_limit: Option<u64>,
    pub daily_council_rank_xp_limit: Option<u64>,
    pub daily_pilot_xp_limit: Option<u64>,
    pub daily_data_running_xp_limit: Option<u64>,
    pub daily_mining_xp_limit: Option<u64>,
    pub daily_crafting_xp_limit: Option<u64>,
    pub items: Option<Vec<ProgressionItemInputUnpacked>>,
}
