use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd71e81805be7f94e")]
pub struct CancelCraftingProcess {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CancelCraftingProcessInstructionAccounts {
    pub funds_to: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub crafting_instance: solana_pubkey::Pubkey,
    pub crafting_process: solana_pubkey::Pubkey,
    pub crafting_facility: solana_pubkey::Pubkey,
    pub game_accounts_and_profile: solana_pubkey::Pubkey,
    pub crafting_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelCraftingProcess {
    type ArrangedAccounts = CancelCraftingProcessInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funds_to = next_account(&mut iter)?;
        let starbase_and_starbase_player = next_account(&mut iter)?;
        let crafting_instance = next_account(&mut iter)?;
        let crafting_process = next_account(&mut iter)?;
        let crafting_facility = next_account(&mut iter)?;
        let game_accounts_and_profile = next_account(&mut iter)?;
        let crafting_program = next_account(&mut iter)?;

        Some(CancelCraftingProcessInstructionAccounts {
            funds_to,
            starbase_and_starbase_player,
            crafting_instance,
            crafting_process,
            crafting_facility,
            game_accounts_and_profile,
            crafting_program,
        })
    }
}
