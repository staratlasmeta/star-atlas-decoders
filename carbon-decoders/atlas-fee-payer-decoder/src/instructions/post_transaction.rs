

use carbon_core::{CarbonDeserialize, borsh};


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
        let [
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
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(PostTransactionInstructionAccounts {
            fee_payer: fee_payer.pubkey,
            rates: rates.pubkey,
            payment_account: payment_account.pubkey,
            token_vault: token_vault.pubkey,
            funder_profile: funder_profile.pubkey,
            funder_key: funder_key.pubkey,
            funder_vault_authority: funder_vault_authority.pubkey,
            funder_vault: funder_vault.pubkey,
            instruction_sysvar: instruction_sysvar.pubkey,
            vault_program: vault_program.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}