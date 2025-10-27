use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x661853308b28ad16")]
pub struct RemoveOutputFromRecipe {
    pub input: RemoveRecipeIngredients,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RemoveOutputFromRecipeInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub recipe: solana_pubkey::Pubkey,
    pub domain: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveOutputFromRecipe {
    type ArrangedAccounts = RemoveOutputFromRecipeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let recipe = next_account(&mut iter)?;
        let domain = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RemoveOutputFromRecipeInstructionAccounts {
            key,
            profile,
            funder,
            recipe,
            domain,
            system_program,
        })
    }
}
