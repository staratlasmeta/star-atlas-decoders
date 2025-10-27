use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x361903475ad7636c")]
pub struct CreateCraftingProcess {
    pub input: CreateCraftingProcessInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateCraftingProcessInstructionAccounts {
    pub location: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub crafting_process: solana_pubkey::Pubkey,
    pub recipe: solana_pubkey::Pubkey,
    pub crafting_facility: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateCraftingProcess {
    type ArrangedAccounts = CreateCraftingProcessInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let location = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let crafting_process = next_account(&mut iter)?;
        let recipe = next_account(&mut iter)?;
        let crafting_facility = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CreateCraftingProcessInstructionAccounts {
            location,
            authority,
            funder,
            crafting_process,
            recipe,
            crafting_facility,
            system_program,
        })
    }
}
