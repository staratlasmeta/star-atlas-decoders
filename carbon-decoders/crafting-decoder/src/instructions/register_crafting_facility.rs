use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xae12c608fe7f7426")]
pub struct RegisterCraftingFacility {
    pub input: RegisterCraftingFacilityInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RegisterCraftingFacilityInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub crafting_facility: solana_pubkey::Pubkey,
    pub domain: solana_pubkey::Pubkey,
    pub location: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RegisterCraftingFacility {
    type ArrangedAccounts = RegisterCraftingFacilityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let crafting_facility = next_account(&mut iter)?;
        let domain = next_account(&mut iter)?;
        let location = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RegisterCraftingFacilityInstructionAccounts {
            key,
            profile,
            funder,
            crafting_facility,
            domain,
            location,
            system_program,
        })
    }
}
