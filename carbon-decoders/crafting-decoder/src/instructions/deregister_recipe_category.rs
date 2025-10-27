use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x678d6598d241e636")]
pub struct DeregisterRecipeCategory {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DeregisterRecipeCategoryInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub recipe_category: solana_pubkey::Pubkey,
    pub domain: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeregisterRecipeCategory {
    type ArrangedAccounts = DeregisterRecipeCategoryInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let funds_to = next_account(&mut iter)?;
        let recipe_category = next_account(&mut iter)?;
        let domain = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(DeregisterRecipeCategoryInstructionAccounts {
            key,
            profile,
            funds_to,
            recipe_category,
            domain,
            system_program,
        })
    }
}
