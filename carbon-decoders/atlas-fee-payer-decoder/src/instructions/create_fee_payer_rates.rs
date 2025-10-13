

use carbon_core::{CarbonDeserialize, borsh};


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
        let [
            rates,
            funder,
            owning_profile,
            token_mint,
            token_owner,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(CreateFeePayerRatesInstructionAccounts {
            rates: rates.pubkey,
            funder: funder.pubkey,
            owning_profile: owning_profile.pubkey,
            token_mint: token_mint.pubkey,
            token_owner: token_owner.pubkey,
            system_program: system_program.pubkey,
        })
    }
}