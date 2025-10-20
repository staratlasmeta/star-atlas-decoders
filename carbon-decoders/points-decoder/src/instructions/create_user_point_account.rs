use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9c94e28d3059298b")]
pub struct CreateUserPointAccount {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateUserPointAccountInstructionAccounts {
    pub user_profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub point_category_account: solana_pubkey::Pubkey,
    pub user_points_account: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateUserPointAccount {
    type ArrangedAccounts = CreateUserPointAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let user_profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let point_category_account = next_account(&mut iter)?;
        let user_points_account = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CreateUserPointAccountInstructionAccounts {
            user_profile,
            funder,
            point_category_account,
            user_points_account,
            system_program,
        })
    }
}
