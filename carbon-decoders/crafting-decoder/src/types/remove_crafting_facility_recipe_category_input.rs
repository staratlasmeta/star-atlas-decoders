use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RemoveCraftingFacilityRecipeCategoryInput {
    pub recipe_category_index: u16,
    pub key_index: u16,
}
