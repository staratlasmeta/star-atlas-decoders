use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x361903475ad7636c")]
pub struct CreateCraftingProcess {
    pub input: StarbaseCreateCraftingProcessInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateCraftingProcessInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    // StarbaseAndStarbasePlayer expansion
    pub starbase: solana_pubkey::Pubkey,
    pub starbase_player: solana_pubkey::Pubkey,
    pub crafting_instance: solana_pubkey::Pubkey,
    pub crafting_facility: solana_pubkey::Pubkey,
    pub crafting_process: solana_pubkey::Pubkey,
    pub crafting_recipe: solana_pubkey::Pubkey,
    pub crafting_domain: solana_pubkey::Pubkey,
    // GameAndGameStateAndProfile expansion
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub profile_faction: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub game_state: solana_pubkey::Pubkey,
    pub crafting_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateCraftingProcess {
    type ArrangedAccounts = CreateCraftingProcessInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funder = next_account(&mut iter)?;

        // StarbaseAndStarbasePlayer expansion
        let starbase = next_account(&mut iter)?;
        let starbase_player = next_account(&mut iter)?;

        let crafting_instance = next_account(&mut iter)?;
        let crafting_facility = next_account(&mut iter)?;
        let crafting_process = next_account(&mut iter)?;
        let crafting_recipe = next_account(&mut iter)?;
        let crafting_domain = next_account(&mut iter)?;

        // GameAndGameStateAndProfile expansion
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let profile_faction = next_account(&mut iter)?;
        let game_id = next_account(&mut iter)?;
        let game_state = next_account(&mut iter)?;

        let crafting_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CreateCraftingProcessInstructionAccounts {
            funder,
            starbase,
            starbase_player,
            crafting_instance,
            crafting_facility,
            crafting_process,
            crafting_recipe,
            crafting_domain,
            key,
            profile,
            profile_faction,
            game_id,
            game_state,
            crafting_program,
            system_program,
        })
    }
}
