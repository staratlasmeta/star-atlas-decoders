use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x27b372c92d34528e")]
pub struct AddCrewToGame {
    pub input: AddCrewInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AddCrewToGameInstructionAccounts {
    pub sage_player_profile: solana_pubkey::Pubkey,
    // StarbaseAndStarbasePlayerMut expansion
    pub starbase: solana_pubkey::Pubkey,
    pub starbase_player: solana_pubkey::Pubkey,
    pub sage_crew_config: solana_pubkey::Pubkey,
    // GameAndProfileAndFaction expansion
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub profile_faction: solana_pubkey::Pubkey,
    pub crew_config: solana_pubkey::Pubkey,
    pub crew_owner: solana_pubkey::Pubkey,
    pub crew_delegate: solana_pubkey::Pubkey,
    pub log_wrapper: solana_pubkey::Pubkey,
    pub compression_program: solana_pubkey::Pubkey,
    pub bubblegum_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddCrewToGame {
    type ArrangedAccounts = AddCrewToGameInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let sage_player_profile = next_account(&mut iter)?;

        // StarbaseAndStarbasePlayerMut expansion
        let starbase = next_account(&mut iter)?;
        let starbase_player = next_account(&mut iter)?;

        let sage_crew_config = next_account(&mut iter)?;

        // GameAndProfileAndFaction expansion
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let profile_faction = next_account(&mut iter)?;

        let crew_config = next_account(&mut iter)?;
        let crew_owner = next_account(&mut iter)?;
        let crew_delegate = next_account(&mut iter)?;
        let log_wrapper = next_account(&mut iter)?;
        let compression_program = next_account(&mut iter)?;
        let bubblegum_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(AddCrewToGameInstructionAccounts {
            sage_player_profile,
            starbase,
            starbase_player,
            sage_crew_config,
            key,
            profile,
            profile_faction,
            crew_config,
            crew_owner,
            crew_delegate,
            log_wrapper,
            compression_program,
            bubblegum_program,
            system_program,
        })
    }
}
