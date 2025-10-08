use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xca15e19c0f046a5d")]
pub struct CloseCraftingProcess {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseCraftingProcessInstructionAccounts {
    pub funds_to: solana_pubkey::Pubkey,
    // StarbaseAndStarbasePlayerMut expansion
    pub starbase: solana_pubkey::Pubkey,
    pub starbase_player: solana_pubkey::Pubkey,
    pub crafting_instance: solana_pubkey::Pubkey,
    pub crafting_process: solana_pubkey::Pubkey,
    pub crafting_recipe: solana_pubkey::Pubkey,
    pub crafting_facility: solana_pubkey::Pubkey,
    // GameAndGameStateAndProfile expansion
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub profile_faction: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub game_state: solana_pubkey::Pubkey,
    // PointsModificationAccounts expansion (crafting)
    pub crafting_user_points_account: solana_pubkey::Pubkey,
    pub crafting_points_category: solana_pubkey::Pubkey,
    pub crafting_points_modifier_account: solana_pubkey::Pubkey,
    // PointsModificationAccounts expansion (council_rank)
    pub council_rank_user_points_account: solana_pubkey::Pubkey,
    pub council_rank_points_category: solana_pubkey::Pubkey,
    pub council_rank_points_modifier_account: solana_pubkey::Pubkey,
    pub progression_config: solana_pubkey::Pubkey,
    pub points_program: solana_pubkey::Pubkey,
    pub crafting_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseCraftingProcess {
    type ArrangedAccounts = CloseCraftingProcessInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funds_to = next_account(&mut iter)?;

        // StarbaseAndStarbasePlayerMut expansion
        let starbase = next_account(&mut iter)?;
        let starbase_player = next_account(&mut iter)?;

        let crafting_instance = next_account(&mut iter)?;
        let crafting_process = next_account(&mut iter)?;
        let crafting_recipe = next_account(&mut iter)?;
        let crafting_facility = next_account(&mut iter)?;

        // GameAndGameStateAndProfile expansion
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let profile_faction = next_account(&mut iter)?;
        let game_id = next_account(&mut iter)?;
        let game_state = next_account(&mut iter)?;

        // PointsModificationAccounts expansion (crafting)
        let crafting_user_points_account = next_account(&mut iter)?;
        let crafting_points_category = next_account(&mut iter)?;
        let crafting_points_modifier_account = next_account(&mut iter)?;

        // PointsModificationAccounts expansion (council_rank)
        let council_rank_user_points_account = next_account(&mut iter)?;
        let council_rank_points_category = next_account(&mut iter)?;
        let council_rank_points_modifier_account = next_account(&mut iter)?;

        let progression_config = next_account(&mut iter)?;
        let points_program = next_account(&mut iter)?;
        let crafting_program = next_account(&mut iter)?;

        Some(CloseCraftingProcessInstructionAccounts {
            funds_to,
            starbase,
            starbase_player,
            crafting_instance,
            crafting_process,
            crafting_recipe,
            crafting_facility,
            key,
            profile,
            profile_faction,
            game_id,
            game_state,
            crafting_user_points_account,
            crafting_points_category,
            crafting_points_modifier_account,
            council_rank_user_points_account,
            council_rank_points_category,
            council_rank_points_modifier_account,
            progression_config,
            points_program,
            crafting_program,
        })
    }
}
