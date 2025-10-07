use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x66da5835ffc2183e")]
pub struct WithdrawCargoFromGame {
    pub input: CargoToGameInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct WithdrawCargoFromGameInstructionAccounts {
    pub funds_to: solana_pubkey::Pubkey,
    // StarbaseAndStarbasePlayer expansion
    pub starbase: solana_pubkey::Pubkey,
    pub starbase_player: solana_pubkey::Pubkey,
    // GameAndGameStateAndProfile expansion
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub profile_faction: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub game_state: solana_pubkey::Pubkey,
    pub cargo_pod: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub token_to: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawCargoFromGame {
    type ArrangedAccounts = WithdrawCargoFromGameInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funds_to = next_account(&mut iter)?;

        // StarbaseAndStarbasePlayer expansion
        let starbase = next_account(&mut iter)?;
        let starbase_player = next_account(&mut iter)?;

        // GameAndGameStateAndProfile expansion
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let profile_faction = next_account(&mut iter)?;
        let game_id = next_account(&mut iter)?;
        let game_state = next_account(&mut iter)?;

        let cargo_pod = next_account(&mut iter)?;
        let cargo_type = next_account(&mut iter)?;
        let cargo_stats_definition = next_account(&mut iter)?;
        let token_from = next_account(&mut iter)?;
        let token_to = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;
        let cargo_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(WithdrawCargoFromGameInstructionAccounts {
            funds_to,
            starbase,
            starbase_player,
            key,
            profile,
            profile_faction,
            game_id,
            game_state,
            cargo_pod,
            cargo_type,
            cargo_stats_definition,
            token_from,
            token_to,
            token_mint,
            cargo_program,
            token_program,
        })
    }
}
