use super::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CreatePointCategoryInput {
    pub license: LicenseType,
    pub point_limit: Option<u64>,
    pub is_spendable: bool,
    pub key_index: u16,
}
