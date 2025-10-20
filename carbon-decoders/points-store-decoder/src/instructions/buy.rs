use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x66063d1201daebea")]
pub struct Buy {
    pub amount: u64,
    pub key_index: u16,
    pub expected_price: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct BuyInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub user_profile: solana_pubkey::Pubkey,
    pub store: solana_pubkey::Pubkey,
    pub store_signer: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
    pub user_points_account: solana_pubkey::Pubkey,
    pub tokens_to: solana_pubkey::Pubkey,
    pub point_category: solana_pubkey::Pubkey,
    pub points_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Buy {
    type ArrangedAccounts = BuyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let user_profile = next_account(&mut iter)?;
        let store = next_account(&mut iter)?;
        let store_signer = next_account(&mut iter)?;
        let bank = next_account(&mut iter)?;
        let user_points_account = next_account(&mut iter)?;
        let tokens_to = next_account(&mut iter)?;
        let point_category = next_account(&mut iter)?;
        let points_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(BuyInstructionAccounts {
            key,
            user_profile,
            store,
            store_signer,
            bank,
            user_points_account,
            tokens_to,
            point_category,
            points_program,
            token_program,
        })
    }
}
