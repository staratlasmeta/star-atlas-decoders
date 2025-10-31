use super::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdatePointCategoryInput {
    pub point_limit: Option<u64>,
    pub new_license: Option<LicenseType>,
    pub is_spendable: Option<bool>,
    pub post_levels_upgrade_threshold: Option<u64>,
    pub key_index: u16,
}
