use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xff213d788877b8eb")]
pub struct DeregisterSurveyDataUnitTracker {
    pub input: DeregisterSurveyDataUnitTrackerInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DeregisterSurveyDataUnitTrackerInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub survey_data_unit_tracker: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeregisterSurveyDataUnitTracker {
    type ArrangedAccounts = DeregisterSurveyDataUnitTrackerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_and_profile = next_account(&mut iter)?;
        let funds_to = next_account(&mut iter)?;
        let survey_data_unit_tracker = next_account(&mut iter)?;

        Some(DeregisterSurveyDataUnitTrackerInstructionAccounts {
            game_and_profile,
            funds_to,
            survey_data_unit_tracker,
        })
    }
}
