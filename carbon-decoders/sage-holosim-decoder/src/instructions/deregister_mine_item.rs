use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x15baa8c854c32949")]
pub struct DeregisterMineItem {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DeregisterMineItemInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub mine_item: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeregisterMineItem {
    type ArrangedAccounts = DeregisterMineItemInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_and_profile = next_account(&mut iter)?;
        let funds_to = next_account(&mut iter)?;
        let mine_item = next_account(&mut iter)?;

        Some(DeregisterMineItemInstructionAccounts {
            game_and_profile,
            funds_to,
            mine_item,
        })
    }
}
