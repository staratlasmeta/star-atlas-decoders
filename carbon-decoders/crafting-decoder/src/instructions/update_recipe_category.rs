use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf900d6c76c53d009")]
pub struct UpdateRecipeCategory {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateRecipeCategoryInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub recipe: solana_pubkey::Pubkey,
    pub recipe_category_old: solana_pubkey::Pubkey,
    pub recipe_category_new: solana_pubkey::Pubkey,
    pub domain: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateRecipeCategory {
    type ArrangedAccounts = UpdateRecipeCategoryInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let recipe = next_account(&mut iter)?;
        let recipe_category_old = next_account(&mut iter)?;
        let recipe_category_new = next_account(&mut iter)?;
        let domain = next_account(&mut iter)?;

        Some(UpdateRecipeCategoryInstructionAccounts {
            key,
            profile,
            recipe,
            recipe_category_old,
            recipe_category_new,
            domain,
        })
    }
}
