use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x404135e37d9903a7")]
pub struct CancelUnstake {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CancelUnstakeInstructionAccounts {
    pub user: solana_pubkey::Pubkey,
    pub registered_stake: solana_pubkey::Pubkey,
    pub staking_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelUnstake {
    type ArrangedAccounts = CancelUnstakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let user = next_account(&mut iter)?;
        let registered_stake = next_account(&mut iter)?;
        let staking_account = next_account(&mut iter)?;

        Some(CancelUnstakeInstructionAccounts {
            user,
            registered_stake,
            staking_account,
        })
    }
}
