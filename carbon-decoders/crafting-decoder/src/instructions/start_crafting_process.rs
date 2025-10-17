use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x406c6d3e09808af6")]
pub struct StartCraftingProcess {
    pub input: StartCraftingProcessInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct StartCraftingProcessInstructionAccounts {
    pub location: solana_pubkey::Pubkey,
    pub crafting_process: solana_pubkey::Pubkey,
    pub recipe: solana_pubkey::Pubkey,
    pub crafting_facility: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for StartCraftingProcess {
    type ArrangedAccounts = StartCraftingProcessInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let location = next_account(&mut iter)?;
        let crafting_process = next_account(&mut iter)?;
        let recipe = next_account(&mut iter)?;
        let crafting_facility = next_account(&mut iter)?;

        Some(StartCraftingProcessInstructionAccounts {
            location,
            crafting_process,
            recipe,
            crafting_facility,
        })
    }
}
