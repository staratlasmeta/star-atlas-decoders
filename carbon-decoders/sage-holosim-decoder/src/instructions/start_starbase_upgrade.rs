use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa5e153a79ed38fcd")]
pub struct StartStarbaseUpgrade {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct StartStarbaseUpgradeInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub upgrade_facility: solana_pubkey::Pubkey,
    pub upgrade_recipe: solana_pubkey::Pubkey,
    pub game_accounts_and_profile: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for StartStarbaseUpgrade {
    type ArrangedAccounts = StartStarbaseUpgradeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funder = next_account(&mut iter)?;
        let starbase_and_starbase_player = next_account(&mut iter)?;
        let upgrade_facility = next_account(&mut iter)?;
        let upgrade_recipe = next_account(&mut iter)?;
        let game_accounts_and_profile = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(StartStarbaseUpgradeInstructionAccounts {
            funder,
            starbase_and_starbase_player,
            upgrade_facility,
            upgrade_recipe,
            game_accounts_and_profile,
            system_program,
        })
    }
}
