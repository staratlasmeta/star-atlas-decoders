use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4ccd42463a551b7f")]
pub struct RemovePointCategoryLevel {
    pub input: RemovePointCategoryLevelInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RemovePointCategoryLevelInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub category: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemovePointCategoryLevel {
    type ArrangedAccounts = RemovePointCategoryLevelInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let funds_to = next_account(&mut iter)?;
        let category = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RemovePointCategoryLevelInstructionAccounts {
            key,
            profile,
            funds_to,
            category,
            system_program,
        })
    }
}
