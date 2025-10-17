use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfd228c9897c0a916")]
pub struct ClaimRecipeOutput {
    pub input: ClaimRecipeOutputInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimRecipeOutputInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub crafting_process: solana_pubkey::Pubkey,
    pub recipe: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub token_to: solana_pubkey::Pubkey,
    pub craftable_item: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimRecipeOutput {
    type ArrangedAccounts = ClaimRecipeOutputInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let authority = next_account(&mut iter)?;
        let crafting_process = next_account(&mut iter)?;
        let recipe = next_account(&mut iter)?;
        let token_from = next_account(&mut iter)?;
        let token_to = next_account(&mut iter)?;
        let craftable_item = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(ClaimRecipeOutputInstructionAccounts {
            authority,
            crafting_process,
            recipe,
            token_from,
            token_to,
            craftable_item,
            token_program,
        })
    }
}
