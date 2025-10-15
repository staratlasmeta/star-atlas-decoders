use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xef2dfb2c230c83a6")]
pub struct PostTransactionNoVault {}

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

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let fee_payer = next_account(&mut iter)?;
        let rates = next_account(&mut iter)?;
        let payment_account = next_account(&mut iter)?;
        let token_vault = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let funder_token_account = next_account(&mut iter)?;
        let instruction_sysvar = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(PostTransactionNoVaultInstructionAccounts {
            fee_payer,
            rates,
            payment_account,
            token_vault,
            funder,
            funder_token_account,
            instruction_sysvar,
            token_program,
            system_program,
        })
    }
}
