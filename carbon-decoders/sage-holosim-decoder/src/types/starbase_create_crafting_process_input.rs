use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct StarbaseCreateCraftingProcessInput {
    pub crafting_id: u64,
    pub recipe_category_index: u16,
    pub quantity: u64,
    pub num_crew: u64,
    pub key_index: u16,
}
