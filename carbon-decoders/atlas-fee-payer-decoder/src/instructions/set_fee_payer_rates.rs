
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh, account_utils::next_account};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe5d15b00ea3243a6")]
pub struct SetFeePayerRates{
    pub set_data: SetFeePayerData,
    pub key_index: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetFeePayerRatesInstructionAccounts {
    pub fee_payer: solana_pubkey::Pubkey,
    pub owning_profile: solana_pubkey::Pubkey,
    pub owning_key: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetFeePayerRates {
    type ArrangedAccounts = SetFeePayerRatesInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let fee_payer = next_account(&mut iter)?;
        let owning_profile = next_account(&mut iter)?;
        let owning_key = next_account(&mut iter)?;

        Some(SetFeePayerRatesInstructionAccounts {
            fee_payer,
            owning_profile,
            owning_key,
        })
    }
}