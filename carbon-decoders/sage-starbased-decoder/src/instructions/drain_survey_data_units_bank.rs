use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe3635637dac960d0")]
pub struct DrainSurveyDataUnitsBank {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DrainSurveyDataUnitsBankInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub survey_data_unit_tracker: solana_pubkey::Pubkey,
    pub survey_data_unit_tracker_signer: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub token_to: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DrainSurveyDataUnitsBank {
    type ArrangedAccounts = DrainSurveyDataUnitsBankInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_and_profile = next_account(&mut iter)?;
        let funds_to = next_account(&mut iter)?;
        let survey_data_unit_tracker = next_account(&mut iter)?;
        let survey_data_unit_tracker_signer = next_account(&mut iter)?;
        let token_from = next_account(&mut iter)?;
        let token_to = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(DrainSurveyDataUnitsBankInstructionAccounts {
            game_and_profile,
            funds_to,
            survey_data_unit_tracker,
            survey_data_unit_tracker_signer,
            token_from,
            token_to,
            token_program,
        })
    }
}
