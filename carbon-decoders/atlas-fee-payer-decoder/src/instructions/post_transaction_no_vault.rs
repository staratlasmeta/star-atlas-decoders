

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xef2dfb2c230c83a6")]
pub struct PostTransactionNoVault{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct PostTransactionNoVaultInstructionAccounts {
    pub fee_payer: solana_pubkey::Pubkey,
    pub rates: solana_pubkey::Pubkey,
    pub payment_account: solana_pubkey::Pubkey,
    pub token_vault: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub funder_token_account: solana_pubkey::Pubkey,
    pub instruction_sysvar: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PostTransactionNoVault {
    type ArrangedAccounts = PostTransactionNoVaultInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            fee_payer,
            rates,
            payment_account,
            token_vault,
            funder,
            funder_token_account,
            instruction_sysvar,
            token_program,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(PostTransactionNoVaultInstructionAccounts {
            fee_payer: fee_payer.pubkey,
            rates: rates.pubkey,
            payment_account: payment_account.pubkey,
            token_vault: token_vault.pubkey,
            funder: funder.pubkey,
            funder_token_account: funder_token_account.pubkey,
            instruction_sysvar: instruction_sysvar.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}