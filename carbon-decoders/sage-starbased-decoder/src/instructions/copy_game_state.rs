use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5f4dfea2f8a81110")]
pub struct CopyGameState {
    pub input: ManageGameInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CopyGameStateInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub old_game_state: solana_pubkey::Pubkey,
    pub new_game_state: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CopyGameState {
    type ArrangedAccounts = CopyGameStateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_and_profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let old_game_state = next_account(&mut iter)?;
        let new_game_state = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CopyGameStateInstructionAccounts {
            game_and_profile,
            funder,
            old_game_state,
            new_game_state,
            system_program,
        })
    }
}
