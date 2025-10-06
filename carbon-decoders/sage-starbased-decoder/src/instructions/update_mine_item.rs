use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe619897592567930")]
pub struct UpdateMineItem {
    pub input: UpdateMineItemInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateMineItemInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub mine_item: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateMineItem {
    type ArrangedAccounts = UpdateMineItemInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_and_profile = next_account(&mut iter)?;
        let mine_item = next_account(&mut iter)?;

        Some(UpdateMineItemInstructionAccounts {
            game_and_profile,
            mine_item,
        })
    }
}
