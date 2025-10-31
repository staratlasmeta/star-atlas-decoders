use super::PointsDecoder;
pub mod add_point_category_level;
pub mod create_point_category;
pub mod create_user_point_account;
pub mod create_user_point_account_with_license;
pub mod decrement_level;
pub mod decrement_points;
pub mod deregister_point_modifier;
pub mod increment_level;
pub mod increment_level_beyond_threshold;
pub mod increment_points;
pub mod register_point_modifier;
pub mod remove_point_category_level;
pub mod spend_points;
pub mod update_point_category;

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
pub enum PointsInstruction {
    AddPointCategoryLevel(add_point_category_level::AddPointCategoryLevel),
    CreatePointCategory(create_point_category::CreatePointCategory),
    CreateUserPointAccount(create_user_point_account::CreateUserPointAccount),
    CreateUserPointAccountWithLicense(
        create_user_point_account_with_license::CreateUserPointAccountWithLicense,
    ),
    DecrementLevel(decrement_level::DecrementLevel),
    DecrementPoints(decrement_points::DecrementPoints),
    DeregisterPointModifier(deregister_point_modifier::DeregisterPointModifier),
    IncrementLevel(increment_level::IncrementLevel),
    IncrementLevelBeyondThreshold(increment_level_beyond_threshold::IncrementLevelBeyondThreshold),
    IncrementPoints(increment_points::IncrementPoints),
    RegisterPointModifier(register_point_modifier::RegisterPointModifier),
    RemovePointCategoryLevel(remove_point_category_level::RemovePointCategoryLevel),
    SpendPoints(spend_points::SpendPoints),
    UpdatePointCategory(update_point_category::UpdatePointCategory),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for PointsDecoder {
    type InstructionType = PointsInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            PointsInstruction::AddPointCategoryLevel => add_point_category_level::AddPointCategoryLevel,
            PointsInstruction::CreatePointCategory => create_point_category::CreatePointCategory,
            PointsInstruction::CreateUserPointAccount => create_user_point_account::CreateUserPointAccount,
            PointsInstruction::CreateUserPointAccountWithLicense => create_user_point_account_with_license::CreateUserPointAccountWithLicense,
            PointsInstruction::DecrementLevel => decrement_level::DecrementLevel,
            PointsInstruction::DecrementPoints => decrement_points::DecrementPoints,
            PointsInstruction::DeregisterPointModifier => deregister_point_modifier::DeregisterPointModifier,
            PointsInstruction::IncrementLevel => increment_level::IncrementLevel,
            PointsInstruction::IncrementLevelBeyondThreshold => increment_level_beyond_threshold::IncrementLevelBeyondThreshold,
            PointsInstruction::IncrementPoints => increment_points::IncrementPoints,
            PointsInstruction::RegisterPointModifier => register_point_modifier::RegisterPointModifier,
            PointsInstruction::RemovePointCategoryLevel => remove_point_category_level::RemovePointCategoryLevel,
            PointsInstruction::SpendPoints => spend_points::SpendPoints,
            PointsInstruction::UpdatePointCategory => update_point_category::UpdatePointCategory,
        )
    }
}
