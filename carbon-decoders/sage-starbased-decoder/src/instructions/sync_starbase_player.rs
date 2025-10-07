use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x785ea4d8a73b03cc")]
pub struct SyncStarbasePlayer {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SyncStarbasePlayerInstructionAccounts {
    // StarbaseAndStarbasePlayerMut expansion
    pub starbase: solana_pubkey::Pubkey,
    pub starbase_player: solana_pubkey::Pubkey,
    // GameAndGameState expansion
    pub game_id: solana_pubkey::Pubkey,
    pub game_state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SyncStarbasePlayer {
    type ArrangedAccounts = SyncStarbasePlayerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        // StarbaseAndStarbasePlayerMut expansion
        let starbase = next_account(&mut iter)?;
        let starbase_player = next_account(&mut iter)?;

        // GameAndGameState expansion
        let game_id = next_account(&mut iter)?;
        let game_state = next_account(&mut iter)?;

        Some(SyncStarbasePlayerInstructionAccounts {
            starbase,
            starbase_player,
            game_id,
            game_state,
        })
    }
}
