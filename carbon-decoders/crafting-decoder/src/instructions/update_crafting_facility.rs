use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb8065c1c8f2bf527")]
pub struct UpdateCraftingFacility {
    pub input: UpdateCraftingFacilityInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateCraftingFacilityInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub crafting_facility: solana_pubkey::Pubkey,
    pub domain: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateCraftingFacility {
    type ArrangedAccounts = UpdateCraftingFacilityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let crafting_facility = next_account(&mut iter)?;
        let domain = next_account(&mut iter)?;

        Some(UpdateCraftingFacilityInstructionAccounts {
            key,
            profile,
            crafting_facility,
            domain,
        })
    }
}
