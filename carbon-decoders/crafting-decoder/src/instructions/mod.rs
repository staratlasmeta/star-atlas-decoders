use super::CraftingDecoder;
pub mod add_consumable_input_to_recipe;
pub mod add_crafting_facility_recipe_category;
pub mod add_non_consumable_input_to_recipe;
pub mod add_output_to_recipe;
pub mod add_recipe_ingredient;
pub mod burn_consumable_ingredient;
pub mod cancel_crafting_process;
pub mod claim_non_consumable_ingredient;
pub mod claim_recipe_output;
pub mod close_crafting_process;
pub mod create_crafting_process;
pub mod deregister_crafting_facility;
pub mod deregister_recipe_category;
pub mod drain_craftable_item_bank;
pub mod initialize_domain;
pub mod legitimize_recipe_ingredient;
pub mod register_craftable_item;
pub mod register_crafting_facility;
pub mod register_recipe;
pub mod register_recipe_category;
pub mod remove_consumable_input_from_recipe;
pub mod remove_crafting_facility_recipe_category;
pub mod remove_non_consumable_input_from_recipe;
pub mod remove_output_from_recipe;
pub mod remove_recipe_ingredient;
pub mod start_crafting_process;
pub mod stop_crafting_process;
pub mod update_crafting_facility;
pub mod update_domain;
pub mod update_recipe;
pub mod update_recipe_category;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum CraftingInstruction {
    AddConsumableInputToRecipe(add_consumable_input_to_recipe::AddConsumableInputToRecipe),
    AddCraftingFacilityRecipeCategory(
        add_crafting_facility_recipe_category::AddCraftingFacilityRecipeCategory,
    ),
    AddNonConsumableInputToRecipe(
        add_non_consumable_input_to_recipe::AddNonConsumableInputToRecipe,
    ),
    AddOutputToRecipe(add_output_to_recipe::AddOutputToRecipe),
    AddRecipeIngredient(add_recipe_ingredient::AddRecipeIngredient),
    BurnConsumableIngredient(burn_consumable_ingredient::BurnConsumableIngredient),
    CancelCraftingProcess(cancel_crafting_process::CancelCraftingProcess),
    ClaimNonConsumableIngredient(claim_non_consumable_ingredient::ClaimNonConsumableIngredient),
    ClaimRecipeOutput(claim_recipe_output::ClaimRecipeOutput),
    CloseCraftingProcess(close_crafting_process::CloseCraftingProcess),
    CreateCraftingProcess(create_crafting_process::CreateCraftingProcess),
    DeregisterCraftingFacility(deregister_crafting_facility::DeregisterCraftingFacility),
    DeregisterRecipeCategory(deregister_recipe_category::DeregisterRecipeCategory),
    DrainCraftableItemBank(drain_craftable_item_bank::DrainCraftableItemBank),
    InitializeDomain(initialize_domain::InitializeDomain),
    LegitimizeRecipeIngredient(legitimize_recipe_ingredient::LegitimizeRecipeIngredient),
    RegisterCraftableItem(register_craftable_item::RegisterCraftableItem),
    RegisterCraftingFacility(register_crafting_facility::RegisterCraftingFacility),
    RegisterRecipe(register_recipe::RegisterRecipe),
    RegisterRecipeCategory(register_recipe_category::RegisterRecipeCategory),
    RemoveConsumableInputFromRecipe(
        remove_consumable_input_from_recipe::RemoveConsumableInputFromRecipe,
    ),
    RemoveCraftingFacilityRecipeCategory(
        remove_crafting_facility_recipe_category::RemoveCraftingFacilityRecipeCategory,
    ),
    RemoveNonConsumableInputFromRecipe(
        remove_non_consumable_input_from_recipe::RemoveNonConsumableInputFromRecipe,
    ),
    RemoveOutputFromRecipe(remove_output_from_recipe::RemoveOutputFromRecipe),
    RemoveRecipeIngredient(remove_recipe_ingredient::RemoveRecipeIngredient),
    StartCraftingProcess(start_crafting_process::StartCraftingProcess),
    StopCraftingProcess(stop_crafting_process::StopCraftingProcess),
    UpdateCraftingFacility(update_crafting_facility::UpdateCraftingFacility),
    UpdateDomain(update_domain::UpdateDomain),
    UpdateRecipe(update_recipe::UpdateRecipe),
    UpdateRecipeCategory(update_recipe_category::UpdateRecipeCategory),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for CraftingDecoder {
    type InstructionType = CraftingInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            CraftingInstruction::AddConsumableInputToRecipe => add_consumable_input_to_recipe::AddConsumableInputToRecipe,
            CraftingInstruction::AddCraftingFacilityRecipeCategory => add_crafting_facility_recipe_category::AddCraftingFacilityRecipeCategory,
            CraftingInstruction::AddNonConsumableInputToRecipe => add_non_consumable_input_to_recipe::AddNonConsumableInputToRecipe,
            CraftingInstruction::AddOutputToRecipe => add_output_to_recipe::AddOutputToRecipe,
            CraftingInstruction::AddRecipeIngredient => add_recipe_ingredient::AddRecipeIngredient,
            CraftingInstruction::BurnConsumableIngredient => burn_consumable_ingredient::BurnConsumableIngredient,
            CraftingInstruction::CancelCraftingProcess => cancel_crafting_process::CancelCraftingProcess,
            CraftingInstruction::ClaimNonConsumableIngredient => claim_non_consumable_ingredient::ClaimNonConsumableIngredient,
            CraftingInstruction::ClaimRecipeOutput => claim_recipe_output::ClaimRecipeOutput,
            CraftingInstruction::CloseCraftingProcess => close_crafting_process::CloseCraftingProcess,
            CraftingInstruction::CreateCraftingProcess => create_crafting_process::CreateCraftingProcess,
            CraftingInstruction::DeregisterCraftingFacility => deregister_crafting_facility::DeregisterCraftingFacility,
            CraftingInstruction::DeregisterRecipeCategory => deregister_recipe_category::DeregisterRecipeCategory,
            CraftingInstruction::DrainCraftableItemBank => drain_craftable_item_bank::DrainCraftableItemBank,
            CraftingInstruction::InitializeDomain => initialize_domain::InitializeDomain,
            CraftingInstruction::LegitimizeRecipeIngredient => legitimize_recipe_ingredient::LegitimizeRecipeIngredient,
            CraftingInstruction::RegisterCraftableItem => register_craftable_item::RegisterCraftableItem,
            CraftingInstruction::RegisterCraftingFacility => register_crafting_facility::RegisterCraftingFacility,
            CraftingInstruction::RegisterRecipe => register_recipe::RegisterRecipe,
            CraftingInstruction::RegisterRecipeCategory => register_recipe_category::RegisterRecipeCategory,
            CraftingInstruction::RemoveConsumableInputFromRecipe => remove_consumable_input_from_recipe::RemoveConsumableInputFromRecipe,
            CraftingInstruction::RemoveCraftingFacilityRecipeCategory => remove_crafting_facility_recipe_category::RemoveCraftingFacilityRecipeCategory,
            CraftingInstruction::RemoveNonConsumableInputFromRecipe => remove_non_consumable_input_from_recipe::RemoveNonConsumableInputFromRecipe,
            CraftingInstruction::RemoveOutputFromRecipe => remove_output_from_recipe::RemoveOutputFromRecipe,
            CraftingInstruction::RemoveRecipeIngredient => remove_recipe_ingredient::RemoveRecipeIngredient,
            CraftingInstruction::StartCraftingProcess => start_crafting_process::StartCraftingProcess,
            CraftingInstruction::StopCraftingProcess => stop_crafting_process::StopCraftingProcess,
            CraftingInstruction::UpdateCraftingFacility => update_crafting_facility::UpdateCraftingFacility,
            CraftingInstruction::UpdateDomain => update_domain::UpdateDomain,
            CraftingInstruction::UpdateRecipe => update_recipe::UpdateRecipe,
            CraftingInstruction::UpdateRecipeCategory => update_recipe_category::UpdateRecipeCategory,
        )
    }
}
