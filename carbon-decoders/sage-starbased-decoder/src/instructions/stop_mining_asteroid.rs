use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb54d2da3671bd351")]
pub struct StopMiningAsteroid {
    pub input: StopMiningAsteroidInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct StopMiningAsteroidInstructionAccounts {
    // GameAndGameStateAndFleetAndOwnerMut expansion
    pub key: solana_pubkey::Pubkey,
    pub owning_profile: solana_pubkey::Pubkey,
    pub owning_profile_faction: solana_pubkey::Pubkey,
    pub fleet: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub game_state: solana_pubkey::Pubkey,
    // Direct accounts
    pub mine_item: solana_pubkey::Pubkey,
    pub resource: solana_pubkey::Pubkey,
    pub planet: solana_pubkey::Pubkey,
    pub fuel_tank: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    // PointsModificationAccounts expansion for pilot_xp_accounts
    pub pilot_user_points_account: solana_pubkey::Pubkey,
    pub pilot_points_category: solana_pubkey::Pubkey,
    pub pilot_points_modifier_account: solana_pubkey::Pubkey,
    // PointsModificationAccounts expansion for mining_xp_accounts
    pub mining_user_points_account: solana_pubkey::Pubkey,
    pub mining_points_category: solana_pubkey::Pubkey,
    pub mining_points_modifier_account: solana_pubkey::Pubkey,
    // PointsModificationAccounts expansion for council_rank_xp_accounts
    pub council_rank_user_points_account: solana_pubkey::Pubkey,
    pub council_rank_points_category: solana_pubkey::Pubkey,
    pub council_rank_points_modifier_account: solana_pubkey::Pubkey,
    // Direct accounts
    pub progression_config: solana_pubkey::Pubkey,
    pub points_program: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for StopMiningAsteroid {
    type ArrangedAccounts = StopMiningAsteroidInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        // GameAndGameStateAndFleetAndOwnerMut expansion
        let key = next_account(&mut iter)?;
        let owning_profile = next_account(&mut iter)?;
        let owning_profile_faction = next_account(&mut iter)?;
        let fleet = next_account(&mut iter)?;
        let game_id = next_account(&mut iter)?;
        let game_state = next_account(&mut iter)?;

        // Direct accounts
        let mine_item = next_account(&mut iter)?;
        let resource = next_account(&mut iter)?;
        let planet = next_account(&mut iter)?;
        let fuel_tank = next_account(&mut iter)?;
        let cargo_type = next_account(&mut iter)?;
        let cargo_stats_definition = next_account(&mut iter)?;
        let token_from = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;

        // PointsModificationAccounts expansion for pilot_xp_accounts
        let pilot_user_points_account = next_account(&mut iter)?;
        let pilot_points_category = next_account(&mut iter)?;
        let pilot_points_modifier_account = next_account(&mut iter)?;

        // PointsModificationAccounts expansion for mining_xp_accounts
        let mining_user_points_account = next_account(&mut iter)?;
        let mining_points_category = next_account(&mut iter)?;
        let mining_points_modifier_account = next_account(&mut iter)?;

        // PointsModificationAccounts expansion for council_rank_xp_accounts
        let council_rank_user_points_account = next_account(&mut iter)?;
        let council_rank_points_category = next_account(&mut iter)?;
        let council_rank_points_modifier_account = next_account(&mut iter)?;

        // Direct accounts
        let progression_config = next_account(&mut iter)?;
        let points_program = next_account(&mut iter)?;
        let cargo_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(StopMiningAsteroidInstructionAccounts {
            key,
            owning_profile,
            owning_profile_faction,
            fleet,
            game_id,
            game_state,
            mine_item,
            resource,
            planet,
            fuel_tank,
            cargo_type,
            cargo_stats_definition,
            token_from,
            token_mint,
            pilot_user_points_account,
            pilot_points_category,
            pilot_points_modifier_account,
            mining_user_points_account,
            mining_points_category,
            mining_points_modifier_account,
            council_rank_user_points_account,
            council_rank_points_category,
            council_rank_points_modifier_account,
            progression_config,
            points_program,
            cargo_program,
            token_program,
        })
    }
}
