use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa9fd02132e4912dc")]
pub struct SyncStarbaseUpgradeIngredients {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SyncStarbaseUpgradeIngredientsInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub starbase: solana_pubkey::Pubkey,
    pub upgrade_recipe: solana_pubkey::Pubkey,
    pub game_accounts_and_profile: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SyncStarbaseUpgradeIngredients {
    type ArrangedAccounts = SyncStarbaseUpgradeIngredientsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funder = next_account(&mut iter)?;
        let starbase = next_account(&mut iter)?;
        let upgrade_recipe = next_account(&mut iter)?;
        let game_accounts_and_profile = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(SyncStarbaseUpgradeIngredientsInstructionAccounts {
            funder,
            starbase,
            upgrade_recipe,
            game_accounts_and_profile,
            system_program,
        })
    }
}
