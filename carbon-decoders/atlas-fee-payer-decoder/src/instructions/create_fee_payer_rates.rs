

use carbon_core::{CarbonDeserialize, borsh, account_utils::next_account};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x1714c8817e5c6c6b")]
pub struct CreateFeePayerRates{
    pub token_limit: u64,
    pub conversion_rate: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateFeePayerRatesInstructionAccounts {
    pub rates: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub owning_profile: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub token_owner: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateFeePayerRates {
    type ArrangedAccounts = CreateFeePayerRatesInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let rates = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let owning_profile = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;
        let token_owner = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CreateFeePayerRatesInstructionAccounts {
            rates,
            funder,
            owning_profile,
            token_mint,
            token_owner,
            system_program,
        })
    }
}