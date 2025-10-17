use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4f768fed4424f20f")]
pub struct UpdateRecipe {
    pub input: UpdateRecipeInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateRecipeInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub recipe: solana_pubkey::Pubkey,
    pub domain: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateRecipe {
    type ArrangedAccounts = UpdateRecipeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let recipe = next_account(&mut iter)?;
        let domain = next_account(&mut iter)?;

        Some(UpdateRecipeInstructionAccounts {
            key,
            profile,
            recipe,
            domain,
        })
    }
}
