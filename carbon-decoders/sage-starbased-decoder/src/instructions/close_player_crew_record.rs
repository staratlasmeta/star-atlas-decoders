use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x42e8136cb4c012e9")]
pub struct ClosePlayerCrewRecord {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClosePlayerCrewRecordInstructionAccounts {
    pub funds_to: solana_pubkey::Pubkey,
    // StarbaseAndStarbasePlayerMut expansion
    pub starbase: solana_pubkey::Pubkey,
    pub starbase_player: solana_pubkey::Pubkey,
    pub crew_record: solana_pubkey::Pubkey,
    // GameAndProfileAndFaction expansion
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub profile_faction: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub crew_config: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClosePlayerCrewRecord {
    type ArrangedAccounts = ClosePlayerCrewRecordInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funds_to = next_account(&mut iter)?;

        // StarbaseAndStarbasePlayerMut expansion
        let starbase = next_account(&mut iter)?;
        let starbase_player = next_account(&mut iter)?;

        let crew_record = next_account(&mut iter)?;

        // GameAndProfileAndFaction expansion
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let profile_faction = next_account(&mut iter)?;
        let game_id = next_account(&mut iter)?;

        let crew_config = next_account(&mut iter)?;

        Some(ClosePlayerCrewRecordInstructionAccounts {
            funds_to,
            starbase,
            starbase_player,
            crew_record,
            key,
            profile,
            profile_faction,
            game_id,
            crew_config,
        })
    }
}
