use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x36ad3a4a80746d14")]
pub struct RegisterMineItem {
    pub input: RegisterMineItemInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RegisterMineItemInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub mine_item: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RegisterMineItem {
    type ArrangedAccounts = RegisterMineItemInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_and_profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let mine_item = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RegisterMineItemInstructionAccounts {
            game_and_profile,
            funder,
            mine_item,
            mint,
            system_program,
        })
    }
}
