

use carbon_core::{CarbonDeserialize, borsh, account_utils::next_account};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x4d5df9b46f54f78d")]
pub struct CreateFeePayer{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateFeePayerInstructionAccounts {
    pub fee_payer: solana_pubkey::Pubkey,
    pub rates: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub payment_account: solana_pubkey::Pubkey,
    pub token_vault: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateFeePayer {
    type ArrangedAccounts = CreateFeePayerInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let fee_payer = next_account(&mut iter)?;
        let rates = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let payment_account = next_account(&mut iter)?;
        let token_vault = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CreateFeePayerInstructionAccounts {
            fee_payer,
            rates,
            funder,
            payment_account,
            token_vault,
            system_program,
        })
    }
}