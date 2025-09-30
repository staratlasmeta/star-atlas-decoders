use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb829fb9a9291c54d")]
pub struct InitializeStaking {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeStakingInstructionAccounts {
    pub update_authority_account: solana_pubkey::Pubkey,
    pub staking_vars_account: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeStaking {
    type ArrangedAccounts = InitializeStakingInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let update_authority_account = next_account(&mut iter)?;
        let staking_vars_account = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(InitializeStakingInstructionAccounts {
            update_authority_account,
            staking_vars_account,
            system_program,
        })
    }
}
