use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4e416ad2a4222225")]
pub struct DeregisterCraftingFacility {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DeregisterCraftingFacilityInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub crafting_facility: solana_pubkey::Pubkey,
    pub domain: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeregisterCraftingFacility {
    type ArrangedAccounts = DeregisterCraftingFacilityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let funds_to = next_account(&mut iter)?;
        let crafting_facility = next_account(&mut iter)?;
        let domain = next_account(&mut iter)?;

        Some(DeregisterCraftingFacilityInstructionAccounts {
            key,
            profile,
            funds_to,
            crafting_facility,
            domain,
        })
    }
}
