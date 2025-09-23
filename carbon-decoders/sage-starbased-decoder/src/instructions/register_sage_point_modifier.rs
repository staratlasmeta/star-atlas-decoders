use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd4fba4b49e13ad16")]
pub struct RegisterSagePointModifier {
    pub input: RegisterSagePointsModifierInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RegisterSagePointModifierInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub points_category: solana_pubkey::Pubkey,
    pub points_modifier: solana_pubkey::Pubkey,
    pub points_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RegisterSagePointModifier {
    type ArrangedAccounts = RegisterSagePointModifierInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_and_profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let points_category = next_account(&mut iter)?;
        let points_modifier = next_account(&mut iter)?;
        let points_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RegisterSagePointModifierInstructionAccounts {
            game_and_profile,
            funder,
            points_category,
            points_modifier,
            points_program,
            system_program,
        })
    }
}
