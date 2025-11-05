use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x83593f62f3d4e066")]
pub struct SpendPoints {
    pub points_amount: u64,
    pub key_index: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SpendPointsInstructionAccounts {
    pub spender: solana_pubkey::Pubkey,
    pub spender_profile: solana_pubkey::Pubkey,
    pub category: solana_pubkey::Pubkey,
    pub user_points_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SpendPoints {
    type ArrangedAccounts = SpendPointsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let spender = next_account(&mut iter)?;
        let spender_profile = next_account(&mut iter)?;
        let category = next_account(&mut iter)?;
        let user_points_account = next_account(&mut iter)?;

        Some(SpendPointsInstructionAccounts {
            spender,
            spender_profile,
            category,
            user_points_account,
        })
    }
}
