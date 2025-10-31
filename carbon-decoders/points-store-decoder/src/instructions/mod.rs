use super::PointsStoreDecoder;
pub mod add_redemption_epoch;
pub mod buy;
pub mod change_store_price;
pub mod claim_tokens;
pub mod close_redemption_config;
pub mod close_store;
pub mod contribute_to_redemption;
pub mod create_points_store;
pub mod create_redemption_config;
pub mod remove_redemption_epoch;
pub mod remove_store_items;
pub mod start_redemption;
pub mod update_redemption_epoch;

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
pub enum PointsStoreInstruction {
    AddRedemptionEpoch(add_redemption_epoch::AddRedemptionEpoch),
    Buy(buy::Buy),
    ChangeStorePrice(change_store_price::ChangeStorePrice),
    ClaimTokens(claim_tokens::ClaimTokens),
    CloseRedemptionConfig(close_redemption_config::CloseRedemptionConfig),
    CloseStore(close_store::CloseStore),
    ContributeToRedemption(contribute_to_redemption::ContributeToRedemption),
    CreatePointsStore(create_points_store::CreatePointsStore),
    CreateRedemptionConfig(create_redemption_config::CreateRedemptionConfig),
    RemoveRedemptionEpoch(remove_redemption_epoch::RemoveRedemptionEpoch),
    RemoveStoreItems(remove_store_items::RemoveStoreItems),
    StartRedemption(start_redemption::StartRedemption),
    UpdateRedemptionEpoch(update_redemption_epoch::UpdateRedemptionEpoch),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for PointsStoreDecoder {
    type InstructionType = PointsStoreInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            PointsStoreInstruction::AddRedemptionEpoch => add_redemption_epoch::AddRedemptionEpoch,
            PointsStoreInstruction::Buy => buy::Buy,
            PointsStoreInstruction::ChangeStorePrice => change_store_price::ChangeStorePrice,
            PointsStoreInstruction::ClaimTokens => claim_tokens::ClaimTokens,
            PointsStoreInstruction::CloseRedemptionConfig => close_redemption_config::CloseRedemptionConfig,
            PointsStoreInstruction::CloseStore => close_store::CloseStore,
            PointsStoreInstruction::ContributeToRedemption => contribute_to_redemption::ContributeToRedemption,
            PointsStoreInstruction::CreatePointsStore => create_points_store::CreatePointsStore,
            PointsStoreInstruction::CreateRedemptionConfig => create_redemption_config::CreateRedemptionConfig,
            PointsStoreInstruction::RemoveRedemptionEpoch => remove_redemption_epoch::RemoveRedemptionEpoch,
            PointsStoreInstruction::RemoveStoreItems => remove_store_items::RemoveStoreItems,
            PointsStoreInstruction::StartRedemption => start_redemption::StartRedemption,
            PointsStoreInstruction::UpdateRedemptionEpoch => update_redemption_epoch::UpdateRedemptionEpoch,
        )
    }
}
