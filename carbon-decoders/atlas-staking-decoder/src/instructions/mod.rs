use super::AtlasStakingDecoder;
pub mod cancel_unstake;
pub mod create_staking_account;
pub mod harvest;
pub mod initialize_staking;
pub mod register_stake;
pub mod settle;
pub mod stake_tokens;
pub mod unstake_tokens;
pub mod update_cooldown_period;
pub mod update_reward_multiplier;
pub mod withdraw_tokens;

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
pub enum AtlasStakingInstruction {
    CancelUnstake(cancel_unstake::CancelUnstake),
    CreateStakingAccount(create_staking_account::CreateStakingAccount),
    Harvest(harvest::Harvest),
    InitializeStaking(initialize_staking::InitializeStaking),
    RegisterStake(register_stake::RegisterStake),
    Settle(settle::Settle),
    StakeTokens(stake_tokens::StakeTokens),
    UnstakeTokens(unstake_tokens::UnstakeTokens),
    UpdateCooldownPeriod(update_cooldown_period::UpdateCooldownPeriod),
    UpdateRewardMultiplier(update_reward_multiplier::UpdateRewardMultiplier),
    WithdrawTokens(withdraw_tokens::WithdrawTokens),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for AtlasStakingDecoder {
    type InstructionType = AtlasStakingInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            AtlasStakingInstruction::CancelUnstake => cancel_unstake::CancelUnstake,
            AtlasStakingInstruction::CreateStakingAccount => create_staking_account::CreateStakingAccount,
            AtlasStakingInstruction::Harvest => harvest::Harvest,
            AtlasStakingInstruction::InitializeStaking => initialize_staking::InitializeStaking,
            AtlasStakingInstruction::RegisterStake => register_stake::RegisterStake,
            AtlasStakingInstruction::Settle => settle::Settle,
            AtlasStakingInstruction::StakeTokens => stake_tokens::StakeTokens,
            AtlasStakingInstruction::UnstakeTokens => unstake_tokens::UnstakeTokens,
            AtlasStakingInstruction::UpdateCooldownPeriod => update_cooldown_period::UpdateCooldownPeriod,
            AtlasStakingInstruction::UpdateRewardMultiplier => update_reward_multiplier::UpdateRewardMultiplier,
            AtlasStakingInstruction::WithdrawTokens => withdraw_tokens::WithdrawTokens,
        )
    }
}
