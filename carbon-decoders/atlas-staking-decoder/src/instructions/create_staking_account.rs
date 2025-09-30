use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf57cdb0dadff9beb")]
pub struct CreateStakingAccount {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateStakingAccountInstructionAccounts {
    pub user: solana_pubkey::Pubkey,
    pub registered_stake: solana_pubkey::Pubkey,
    pub staking_account: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateStakingAccount {
    type ArrangedAccounts = CreateStakingAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let user = next_account(&mut iter)?;
        let registered_stake = next_account(&mut iter)?;
        let staking_account = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CreateStakingAccountInstructionAccounts {
            user,
            registered_stake,
            staking_account,
            system_program,
        })
    }
}
