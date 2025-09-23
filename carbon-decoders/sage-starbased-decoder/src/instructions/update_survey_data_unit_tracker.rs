use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xce1bf719ddcfdb23")]
pub struct UpdateSurveyDataUnitTracker {
    pub input: UpdateSurveyDataUnitTrackerInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateSurveyDataUnitTrackerInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub survey_data_unit_tracker: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateSurveyDataUnitTracker {
    type ArrangedAccounts = UpdateSurveyDataUnitTrackerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_and_profile = next_account(&mut iter)?;
        let survey_data_unit_tracker = next_account(&mut iter)?;

        Some(UpdateSurveyDataUnitTrackerInstructionAccounts {
            game_and_profile,
            survey_data_unit_tracker,
        })
    }
}
