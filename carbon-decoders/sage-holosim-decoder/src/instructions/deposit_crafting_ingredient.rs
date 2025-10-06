use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x202391d538ffd19c")]
pub struct DepositCraftingIngredient {
    pub input: StarbaseDepositCraftingIngredientInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DepositCraftingIngredientInstructionAccounts {
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub crafting_instance: solana_pubkey::Pubkey,
    pub crafting_facility: solana_pubkey::Pubkey,
    pub crafting_process: solana_pubkey::Pubkey,
    pub cargo_pod_from: solana_pubkey::Pubkey,
    pub crafting_recipe: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub game_accounts_and_profile: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub token_to: solana_pubkey::Pubkey,
    pub crafting_program: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositCraftingIngredient {
    type ArrangedAccounts = DepositCraftingIngredientInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let starbase_and_starbase_player = next_account(&mut iter)?;
        let crafting_instance = next_account(&mut iter)?;
        let crafting_facility = next_account(&mut iter)?;
        let crafting_process = next_account(&mut iter)?;
        let cargo_pod_from = next_account(&mut iter)?;
        let crafting_recipe = next_account(&mut iter)?;
        let cargo_type = next_account(&mut iter)?;
        let cargo_stats_definition = next_account(&mut iter)?;
        let game_accounts_and_profile = next_account(&mut iter)?;
        let token_from = next_account(&mut iter)?;
        let token_to = next_account(&mut iter)?;
        let crafting_program = next_account(&mut iter)?;
        let cargo_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(DepositCraftingIngredientInstructionAccounts {
            starbase_and_starbase_player,
            crafting_instance,
            crafting_facility,
            crafting_process,
            cargo_pod_from,
            crafting_recipe,
            cargo_type,
            cargo_stats_definition,
            game_accounts_and_profile,
            token_from,
            token_to,
            crafting_program,
            cargo_program,
            token_program,
        })
    }
}
