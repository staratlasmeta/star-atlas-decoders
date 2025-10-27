use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct StartCraftingProcessInput {
    pub recipe_duration_override: Option<u64>,
    pub local_time: Option<i64>,
}
