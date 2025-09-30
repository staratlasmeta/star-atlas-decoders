use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd6848a7b88115952")]
pub struct CloseUpgradeProcess {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseUpgradeProcessInstructionAccounts {
    pub funds_to: solana_pubkey::Pubkey,
    // StarbaseMutAndStarbasePlayer expansion
    pub starbase: solana_pubkey::Pubkey,
    pub starbase_player: solana_pubkey::Pubkey,
    // Direct accounts
    pub resource_crafting_instance: solana_pubkey::Pubkey,
    pub resource_crafting_process: solana_pubkey::Pubkey,
    pub resource_recipe: solana_pubkey::Pubkey,
    pub resource_crafting_facility: solana_pubkey::Pubkey,
    // GameAndGameStateAndProfile expansion
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub profile_faction: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub game_state: solana_pubkey::Pubkey,
    // Direct account
    pub crafting_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseUpgradeProcess {
    type ArrangedAccounts = CloseUpgradeProcessInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funds_to = next_account(&mut iter)?;

        // StarbaseMutAndStarbasePlayer expansion
        let starbase = next_account(&mut iter)?;
        let starbase_player = next_account(&mut iter)?;

        // Direct accounts
        let resource_crafting_instance = next_account(&mut iter)?;
        let resource_crafting_process = next_account(&mut iter)?;
        let resource_recipe = next_account(&mut iter)?;
        let resource_crafting_facility = next_account(&mut iter)?;

        // GameAndGameStateAndProfile expansion
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let profile_faction = next_account(&mut iter)?;
        let game_id = next_account(&mut iter)?;
        let game_state = next_account(&mut iter)?;

        // Direct account
        let crafting_program = next_account(&mut iter)?;

        Some(CloseUpgradeProcessInstructionAccounts {
            funds_to,
            starbase,
            starbase_player,
            resource_crafting_instance,
            resource_crafting_process,
            resource_recipe,
            resource_crafting_facility,
            key,
            profile,
            profile_faction,
            game_id,
            game_state,
            crafting_program,
        })
    }
}
