use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CreateCraftingProcessInput {
    pub crafting_id: u64,
    pub recipe_category_index: u16,
    pub quantity: u64,
    pub deny_permissionless_claiming: bool,
    pub use_local_time: bool,
}
