use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ClaimRecipeOutputInput {
    pub ingredient_index: u16,
    pub local_time: Option<i64>,
}
