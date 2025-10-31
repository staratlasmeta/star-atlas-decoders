use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xafd893f54031b1e1")]
pub struct IncrementPoints {
    pub input: IncrementPointsInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct IncrementPointsInstructionAccounts {
    pub category: solana_pubkey::Pubkey,
    pub user_points_account: solana_pubkey::Pubkey,
    pub points_modifier_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for IncrementPoints {
    type ArrangedAccounts = IncrementPointsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let category = next_account(&mut iter)?;
        let user_points_account = next_account(&mut iter)?;
        let points_modifier_account = next_account(&mut iter)?;

        Some(IncrementPointsInstructionAccounts {
            category,
            user_points_account,
            points_modifier_account,
        })
    }
}
