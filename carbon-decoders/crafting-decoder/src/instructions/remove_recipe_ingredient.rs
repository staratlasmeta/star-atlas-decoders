use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe91d0d16773f5acf")]
pub struct RemoveRecipeIngredient {
    pub input: RemoveRecipeIngredientInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RemoveRecipeIngredientInstructionAccounts {
    pub location: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub crafting_process: solana_pubkey::Pubkey,
    pub recipe: solana_pubkey::Pubkey,
    pub crafting_facility: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub token_to: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveRecipeIngredient {
    type ArrangedAccounts = RemoveRecipeIngredientInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let location = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let crafting_process = next_account(&mut iter)?;
        let recipe = next_account(&mut iter)?;
        let crafting_facility = next_account(&mut iter)?;
        let token_from = next_account(&mut iter)?;
        let token_to = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(RemoveRecipeIngredientInstructionAccounts {
            location,
            authority,
            crafting_process,
            recipe,
            crafting_facility,
            token_from,
            token_to,
            mint,
            token_program,
        })
    }
}
