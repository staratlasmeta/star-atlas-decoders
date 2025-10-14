

use carbon_core::{CarbonDeserialize, borsh, account_utils::next_account};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x381d7fcf2b5f6366")]
pub struct PostTransaction{
    pub funder_key_index: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct PostTransactionInstructionAccounts {
    pub fee_payer: solana_pubkey::Pubkey,
    pub rates: solana_pubkey::Pubkey,
    pub payment_account: solana_pubkey::Pubkey,
    pub token_vault: solana_pubkey::Pubkey,
    pub funder_profile: solana_pubkey::Pubkey,
    pub funder_key: solana_pubkey::Pubkey,
    pub funder_vault_authority: solana_pubkey::Pubkey,
    pub funder_vault: solana_pubkey::Pubkey,
    pub instruction_sysvar: solana_pubkey::Pubkey,
    pub vault_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PostTransaction {
    type ArrangedAccounts = PostTransactionInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let fee_payer = next_account(&mut iter)?;
        let rates = next_account(&mut iter)?;
        let payment_account = next_account(&mut iter)?;
        let token_vault = next_account(&mut iter)?;
        let funder_profile = next_account(&mut iter)?;
        let funder_key = next_account(&mut iter)?;
        let funder_vault_authority = next_account(&mut iter)?;
        let funder_vault = next_account(&mut iter)?;
        let instruction_sysvar = next_account(&mut iter)?;
        let vault_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(PostTransactionInstructionAccounts {
            fee_payer,
            rates,
            payment_account,
            token_vault,
            funder_profile,
            funder_key,
            funder_vault_authority,
            funder_vault,
            instruction_sysvar,
            vault_program,
            token_program,
            system_program,
        })
    }
}