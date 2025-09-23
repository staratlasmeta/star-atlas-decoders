use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SubmitStarbaseUpgradeResourceInput {
    pub points_program_permissions_key_index: u16,
    pub sage_permissions_key_index: u16,
    pub upgrade_process_recipe_input_index: u16,
    pub starbase_upgrade_recipe_input_index: u16,
    pub resource_recipe_output_index: u16,
    pub epoch_index: u16,
}
