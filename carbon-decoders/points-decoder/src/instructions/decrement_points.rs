use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x418795193e215307")]
pub struct DecrementPoints {
    pub points_amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DecrementPointsInstructionAccounts {
    pub category: solana_pubkey::Pubkey,
    pub user_points_account: solana_pubkey::Pubkey,
    pub points_modifier_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DecrementPoints {
    type ArrangedAccounts = DecrementPointsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let category = next_account(&mut iter)?;
        let user_points_account = next_account(&mut iter)?;
        let points_modifier_account = next_account(&mut iter)?;

        Some(DecrementPointsInstructionAccounts {
            category,
            user_points_account,
            points_modifier_account,
        })
    }
}
