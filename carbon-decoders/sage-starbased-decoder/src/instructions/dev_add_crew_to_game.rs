use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x50f4294448a91acf")]
pub struct DevAddCrewToGame {
    pub input: DevAddCrewInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DevAddCrewToGameInstructionAccounts {
    // GameAndProfile expansion
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub starbase_player: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DevAddCrewToGame {
    type ArrangedAccounts = DevAddCrewToGameInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        // GameAndProfile expansion
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let game_id = next_account(&mut iter)?;

        let starbase_player = next_account(&mut iter)?;

        Some(DevAddCrewToGameInstructionAccounts {
            key,
            profile,
            game_id,
            starbase_player,
        })
    }
}
