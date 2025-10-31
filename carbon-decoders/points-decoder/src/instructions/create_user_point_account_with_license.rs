use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb44cc25b9683acbc")]
pub struct CreateUserPointAccountWithLicense {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateUserPointAccountWithLicenseInstructionAccounts {
    pub user_profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub category: solana_pubkey::Pubkey,
    pub user_points_account: solana_pubkey::Pubkey,
    pub token_account_owner: solana_pubkey::Pubkey,
    pub user_token_account: solana_pubkey::Pubkey,
    pub mint_or_vault: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateUserPointAccountWithLicense {
    type ArrangedAccounts = CreateUserPointAccountWithLicenseInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let user_profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let category = next_account(&mut iter)?;
        let user_points_account = next_account(&mut iter)?;
        let token_account_owner = next_account(&mut iter)?;
        let user_token_account = next_account(&mut iter)?;
        let mint_or_vault = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CreateUserPointAccountWithLicenseInstructionAccounts {
            user_profile,
            funder,
            category,
            user_points_account,
            token_account_owner,
            user_token_account,
            mint_or_vault,
            token_program,
            system_program,
        })
    }
}
