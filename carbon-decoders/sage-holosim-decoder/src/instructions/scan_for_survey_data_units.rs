use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5466ea017e88ba93")]
pub struct ScanForSurveyDataUnits {
    pub input: ScanForSurveyDataUnitsInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ScanForSurveyDataUnitsInstructionAccounts {
    // GameAndGameStateAndFleetAndOwnerMut expansion
    pub key: solana_pubkey::Pubkey,
    pub owning_profile: solana_pubkey::Pubkey,
    pub owning_profile_faction: solana_pubkey::Pubkey,
    pub fleet: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub game_state: solana_pubkey::Pubkey,
    pub survey_data_unit_tracker: solana_pubkey::Pubkey,
    pub survey_data_unit_tracker_signer: solana_pubkey::Pubkey,
    pub cargo_hold: solana_pubkey::Pubkey,
    pub sector: solana_pubkey::Pubkey,
    pub sdu_token_from: solana_pubkey::Pubkey,
    pub sdu_token_to: solana_pubkey::Pubkey,
    pub resource_token_from: solana_pubkey::Pubkey,
    pub resource_mint: solana_pubkey::Pubkey,
    pub sdu_cargo_type: solana_pubkey::Pubkey,
    pub resource_cargo_type: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    // PointsModificationAccounts expansion (data_running)
    pub data_running_user_points_account: solana_pubkey::Pubkey,
    pub data_running_points_category: solana_pubkey::Pubkey,
    pub data_running_points_modifier_account: solana_pubkey::Pubkey,
    // PointsModificationAccounts expansion (council_rank)
    pub council_rank_user_points_account: solana_pubkey::Pubkey,
    pub council_rank_points_category: solana_pubkey::Pubkey,
    pub council_rank_points_modifier_account: solana_pubkey::Pubkey,
    pub progression_config: solana_pubkey::Pubkey,
    pub points_program: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub instructions_sysvar: solana_pubkey::Pubkey,
    pub recent_slothashes: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ScanForSurveyDataUnits {
    type ArrangedAccounts = ScanForSurveyDataUnitsInstructionAccounts;

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

        let survey_data_unit_tracker = next_account(&mut iter)?;
        let survey_data_unit_tracker_signer = next_account(&mut iter)?;
        let cargo_hold = next_account(&mut iter)?;
        let sector = next_account(&mut iter)?;
        let sdu_token_from = next_account(&mut iter)?;
        let sdu_token_to = next_account(&mut iter)?;
        let resource_token_from = next_account(&mut iter)?;
        let resource_mint = next_account(&mut iter)?;
        let sdu_cargo_type = next_account(&mut iter)?;
        let resource_cargo_type = next_account(&mut iter)?;
        let cargo_stats_definition = next_account(&mut iter)?;

        // PointsModificationAccounts expansion (data_running)
        let data_running_user_points_account = next_account(&mut iter)?;
        let data_running_points_category = next_account(&mut iter)?;
        let data_running_points_modifier_account = next_account(&mut iter)?;

        // PointsModificationAccounts expansion (council_rank)
        let council_rank_user_points_account = next_account(&mut iter)?;
        let council_rank_points_category = next_account(&mut iter)?;
        let council_rank_points_modifier_account = next_account(&mut iter)?;

        let progression_config = next_account(&mut iter)?;
        let points_program = next_account(&mut iter)?;
        let cargo_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let instructions_sysvar = next_account(&mut iter)?;
        let recent_slothashes = next_account(&mut iter)?;

        Some(ScanForSurveyDataUnitsInstructionAccounts {
            key,
            owning_profile,
            owning_profile_faction,
            fleet,
            game_id,
            game_state,
            survey_data_unit_tracker,
            survey_data_unit_tracker_signer,
            cargo_hold,
            sector,
            sdu_token_from,
            sdu_token_to,
            resource_token_from,
            resource_mint,
            sdu_cargo_type,
            resource_cargo_type,
            cargo_stats_definition,
            data_running_user_points_account,
            data_running_points_category,
            data_running_points_modifier_account,
            council_rank_user_points_account,
            council_rank_points_category,
            council_rank_points_modifier_account,
            progression_config,
            points_program,
            cargo_program,
            token_program,
            instructions_sysvar,
            recent_slothashes,
        })
    }
}
