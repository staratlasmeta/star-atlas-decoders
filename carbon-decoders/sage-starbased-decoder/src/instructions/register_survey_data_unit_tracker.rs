use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd048633f00d6289b")]
pub struct RegisterSurveyDataUnitTracker {
    pub input: RegisterSurveyDataUnitTrackerInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RegisterSurveyDataUnitTrackerInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub survey_data_unit_tracker: solana_pubkey::Pubkey,
    pub sdu_mint: solana_pubkey::Pubkey,
    pub resource_mint: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RegisterSurveyDataUnitTracker {
    type ArrangedAccounts = RegisterSurveyDataUnitTrackerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_and_profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let survey_data_unit_tracker = next_account(&mut iter)?;
        let sdu_mint = next_account(&mut iter)?;
        let resource_mint = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RegisterSurveyDataUnitTrackerInstructionAccounts {
            game_and_profile,
            funder,
            survey_data_unit_tracker,
            sdu_mint,
            resource_mint,
            system_program,
        })
    }
}
