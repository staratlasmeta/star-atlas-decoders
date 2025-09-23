use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x31c0acf44f2caab2")]
pub struct StopCraftingProcess {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct StopCraftingProcessInstructionAccounts {
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub crafting_instance: solana_pubkey::Pubkey,
    pub crafting_process: solana_pubkey::Pubkey,
    pub crafting_recipe: solana_pubkey::Pubkey,
    pub crafting_facility: solana_pubkey::Pubkey,
    pub game_accounts_and_profile: solana_pubkey::Pubkey,
    pub crafting_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for StopCraftingProcess {
    type ArrangedAccounts = StopCraftingProcessInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let starbase_and_starbase_player = next_account(&mut iter)?;
        let crafting_instance = next_account(&mut iter)?;
        let crafting_process = next_account(&mut iter)?;
        let crafting_recipe = next_account(&mut iter)?;
        let crafting_facility = next_account(&mut iter)?;
        let game_accounts_and_profile = next_account(&mut iter)?;
        let crafting_program = next_account(&mut iter)?;

        Some(StopCraftingProcessInstructionAccounts {
            starbase_and_starbase_player,
            crafting_instance,
            crafting_process,
            crafting_recipe,
            crafting_facility,
            game_accounts_and_profile,
            crafting_program,
        })
    }
}
