use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa0e9df303e6b47b0")]
pub struct CreatePointCategory {
    pub input: CreatePointCategoryInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreatePointCategoryInstructionAccounts {
    pub profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub category: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreatePointCategory {
    type ArrangedAccounts = CreatePointCategoryInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let category = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CreatePointCategoryInstructionAccounts {
            profile,
            funder,
            category,
            system_program,
        })
    }
}
