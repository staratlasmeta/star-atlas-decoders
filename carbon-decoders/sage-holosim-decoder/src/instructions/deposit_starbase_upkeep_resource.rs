use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb0a00bfa22425e0c")]
pub struct DepositStarbaseUpkeepResource {
    pub input: DepositStarbaseUpkeepResourceInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DepositStarbaseUpkeepResourceInstructionAccounts {
    pub funds_to: solana_pubkey::Pubkey,
    // StarbaseMutAndStarbasePlayer expansion
    pub starbase: solana_pubkey::Pubkey,
    pub starbase_player: solana_pubkey::Pubkey,
    pub cargo_pod_from: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    // GameAndGameStateAndProfile expansion
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub profile_faction: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub game_state: solana_pubkey::Pubkey,
    pub resource_recipe: solana_pubkey::Pubkey,
    // PointsModificationAccounts expansion (loyalty)
    pub loyalty_user_points_account: solana_pubkey::Pubkey,
    pub loyalty_points_category: solana_pubkey::Pubkey,
    pub loyalty_points_modifier_account: solana_pubkey::Pubkey,
    pub progression_config: solana_pubkey::Pubkey,
    pub points_program: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositStarbaseUpkeepResource {
    type ArrangedAccounts = DepositStarbaseUpkeepResourceInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funds_to = next_account(&mut iter)?;

        // StarbaseMutAndStarbasePlayer expansion
        let starbase = next_account(&mut iter)?;
        let starbase_player = next_account(&mut iter)?;

        let cargo_pod_from = next_account(&mut iter)?;
        let cargo_type = next_account(&mut iter)?;
        let cargo_stats_definition = next_account(&mut iter)?;
        let token_from = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;

        // GameAndGameStateAndProfile expansion
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let profile_faction = next_account(&mut iter)?;
        let game_id = next_account(&mut iter)?;
        let game_state = next_account(&mut iter)?;

        let resource_recipe = next_account(&mut iter)?;

        // PointsModificationAccounts expansion (loyalty)
        let loyalty_user_points_account = next_account(&mut iter)?;
        let loyalty_points_category = next_account(&mut iter)?;
        let loyalty_points_modifier_account = next_account(&mut iter)?;

        let progression_config = next_account(&mut iter)?;
        let points_program = next_account(&mut iter)?;
        let cargo_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(DepositStarbaseUpkeepResourceInstructionAccounts {
            funds_to,
            starbase,
            starbase_player,
            cargo_pod_from,
            cargo_type,
            cargo_stats_definition,
            token_from,
            token_mint,
            key,
            profile,
            profile_faction,
            game_id,
            game_state,
            resource_recipe,
            loyalty_user_points_account,
            loyalty_points_category,
            loyalty_points_modifier_account,
            progression_config,
            points_program,
            cargo_program,
            token_program,
        })
    }
}
