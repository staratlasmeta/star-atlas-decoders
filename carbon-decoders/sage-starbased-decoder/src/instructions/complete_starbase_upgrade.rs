use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8f5cc0f9156cad51")]
pub struct CompleteStarbaseUpgrade {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CompleteStarbaseUpgradeInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    // StarbaseMutAndStarbasePlayer expansion
    pub starbase: solana_pubkey::Pubkey,
    pub starbase_player: solana_pubkey::Pubkey,
    // Direct accounts
    pub crafting_facility: solana_pubkey::Pubkey,
    pub upgrade_facility: solana_pubkey::Pubkey,
    pub upgrade_recipe: solana_pubkey::Pubkey,
    pub new_recipe_category: solana_pubkey::Pubkey,
    pub crafting_domain: solana_pubkey::Pubkey,
    // GameAndGameStateAndProfile expansion
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub profile_faction: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub game_state: solana_pubkey::Pubkey,
    // Direct accounts
    pub crafting_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CompleteStarbaseUpgrade {
    type ArrangedAccounts = CompleteStarbaseUpgradeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funder = next_account(&mut iter)?;

        // StarbaseMutAndStarbasePlayer expansion
        let starbase = next_account(&mut iter)?;
        let starbase_player = next_account(&mut iter)?;

        // Direct accounts
        let crafting_facility = next_account(&mut iter)?;
        let upgrade_facility = next_account(&mut iter)?;
        let upgrade_recipe = next_account(&mut iter)?;
        let new_recipe_category = next_account(&mut iter)?;
        let crafting_domain = next_account(&mut iter)?;

        // GameAndGameStateAndProfile expansion
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let profile_faction = next_account(&mut iter)?;
        let game_id = next_account(&mut iter)?;
        let game_state = next_account(&mut iter)?;

        // Direct accounts
        let crafting_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CompleteStarbaseUpgradeInstructionAccounts {
            funder,
            starbase,
            starbase_player,
            crafting_facility,
            upgrade_facility,
            upgrade_recipe,
            new_recipe_category,
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
