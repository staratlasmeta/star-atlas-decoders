use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0de1cb5b36e87eaa")]
pub struct BurnCraftingConsumables {
    pub input: IngredientIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct BurnCraftingConsumablesInstructionAccounts {
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub crafting_instance: solana_pubkey::Pubkey,
    pub crafting_process: solana_pubkey::Pubkey,
    pub crafting_facility: solana_pubkey::Pubkey,
    pub crafting_recipe: solana_pubkey::Pubkey,
    pub game_accounts: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub crafting_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BurnCraftingConsumables {
    type ArrangedAccounts = BurnCraftingConsumablesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let starbase_and_starbase_player = next_account(&mut iter)?;
        let crafting_instance = next_account(&mut iter)?;
        let crafting_process = next_account(&mut iter)?;
        let crafting_facility = next_account(&mut iter)?;
        let crafting_recipe = next_account(&mut iter)?;
        let game_accounts = next_account(&mut iter)?;
        let token_from = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;
        let crafting_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(BurnCraftingConsumablesInstructionAccounts {
            starbase_and_starbase_player,
            crafting_instance,
            crafting_process,
            crafting_facility,
            crafting_recipe,
            game_accounts,
            token_from,
            token_mint,
            crafting_program,
            token_program,
        })
    }
}
