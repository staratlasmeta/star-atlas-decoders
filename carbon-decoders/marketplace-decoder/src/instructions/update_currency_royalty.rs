use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb3e8052acc5aaef8")]
pub struct UpdateCurrencyRoyalty {
    pub royalty: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateCurrencyRoyaltyInstructionAccounts {
    pub update_authority_account: solana_pubkey::Pubkey,
    pub market_vars_account: solana_pubkey::Pubkey,
    pub registered_currency: solana_pubkey::Pubkey,
    pub currency_mint: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateCurrencyRoyalty {
    type ArrangedAccounts = UpdateCurrencyRoyaltyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let update_authority_account = next_account(&mut iter)?;
        let market_vars_account = next_account(&mut iter)?;
        let registered_currency = next_account(&mut iter)?;
        let currency_mint = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(UpdateCurrencyRoyaltyInstructionAccounts {
            update_authority_account,
            market_vars_account,
            registered_currency,
            currency_mint,
            system_program,
        })
    }
}
