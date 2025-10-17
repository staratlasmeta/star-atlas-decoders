use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd5fd014bea1c7d16")]
pub struct RemoveCraftingFacilityRecipeCategory {
    pub input: RemoveCraftingFacilityRecipeCategoryInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RemoveCraftingFacilityRecipeCategoryInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub recipe_category: solana_pubkey::Pubkey,
    pub crafting_facility: solana_pubkey::Pubkey,
    pub domain: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveCraftingFacilityRecipeCategory {
    type ArrangedAccounts = RemoveCraftingFacilityRecipeCategoryInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let recipe_category = next_account(&mut iter)?;
        let crafting_facility = next_account(&mut iter)?;
        let domain = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RemoveCraftingFacilityRecipeCategoryInstructionAccounts {
            key,
            profile,
            funder,
            recipe_category,
            crafting_facility,
            domain,
            system_program,
        })
    }
}
