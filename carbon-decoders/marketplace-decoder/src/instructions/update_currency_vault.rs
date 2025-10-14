use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1288481f4cf20a52")]
pub struct UpdateCurrencyVault {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateCurrencyVaultInstructionAccounts {
    pub update_authority_account: solana_pubkey::Pubkey,
    pub market_vars_account: solana_pubkey::Pubkey,
    pub registered_currency: solana_pubkey::Pubkey,
    pub currency_mint: solana_pubkey::Pubkey,
    pub sa_currency_vault: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateCurrencyVault {
    type ArrangedAccounts = UpdateCurrencyVaultInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let update_authority_account = next_account(&mut iter)?;
        let market_vars_account = next_account(&mut iter)?;
        let registered_currency = next_account(&mut iter)?;
        let currency_mint = next_account(&mut iter)?;
        let sa_currency_vault = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(UpdateCurrencyVaultInstructionAccounts {
            update_authority_account,
            market_vars_account,
            registered_currency,
            currency_mint,
            sa_currency_vault,
            system_program,
        })
    }
}
