use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf28b4c0f17b66475")]
pub struct RemoveCrewFromGame {
    pub input: RemoveCrewInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RemoveCrewFromGameInstructionAccounts {
    pub sage_player_profile: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub sage_crew_config: solana_pubkey::Pubkey,
    pub game_and_profile_and_faction: solana_pubkey::Pubkey,
    pub crew_config: solana_pubkey::Pubkey,
    pub new_crew_owner: solana_pubkey::Pubkey,
    pub crew_delegate: solana_pubkey::Pubkey,
    pub log_wrapper: solana_pubkey::Pubkey,
    pub compression_program: solana_pubkey::Pubkey,
    pub bubblegum_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveCrewFromGame {
    type ArrangedAccounts = RemoveCrewFromGameInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let sage_player_profile = next_account(&mut iter)?;
        let starbase_and_starbase_player = next_account(&mut iter)?;
        let sage_crew_config = next_account(&mut iter)?;
        let game_and_profile_and_faction = next_account(&mut iter)?;
        let crew_config = next_account(&mut iter)?;
        let new_crew_owner = next_account(&mut iter)?;
        let crew_delegate = next_account(&mut iter)?;
        let log_wrapper = next_account(&mut iter)?;
        let compression_program = next_account(&mut iter)?;
        let bubblegum_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RemoveCrewFromGameInstructionAccounts {
            sage_player_profile,
            starbase_and_starbase_player,
            sage_crew_config,
            game_and_profile_and_faction,
            crew_config,
            new_crew_owner,
            crew_delegate,
            log_wrapper,
            compression_program,
            bubblegum_program,
            system_program,
        })
    }
}
