use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x80f05dcbe40e659a")]
pub struct DecrementLevel {
    pub input: DecrementLevelInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DecrementLevelInstructionAccounts {
    pub category: solana_pubkey::Pubkey,
    pub user_points_account: solana_pubkey::Pubkey,
    pub points_modifier_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DecrementLevel {
    type ArrangedAccounts = DecrementLevelInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let category = next_account(&mut iter)?;
        let user_points_account = next_account(&mut iter)?;
        let points_modifier_account = next_account(&mut iter)?;

        Some(DecrementLevelInstructionAccounts {
            category,
            user_points_account,
            points_modifier_account,
        })
    }
}
