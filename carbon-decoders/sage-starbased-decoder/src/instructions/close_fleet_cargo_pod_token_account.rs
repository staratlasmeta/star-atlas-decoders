use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4e7cf7a394fd8e44")]
pub struct CloseFleetCargoPodTokenAccount {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseFleetCargoPodTokenAccountInstructionAccounts {
    // GameAndGameStateAndFleetAndOwnerMut expansion
    pub key: solana_pubkey::Pubkey,
    pub owning_profile: solana_pubkey::Pubkey,
    pub owning_profile_faction: solana_pubkey::Pubkey,
    pub fleet: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub game_state: solana_pubkey::Pubkey,
    // Direct accounts
    pub cargo_pod: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub token: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseFleetCargoPodTokenAccount {
    type ArrangedAccounts = CloseFleetCargoPodTokenAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        // GameAndGameStateAndFleetAndOwnerMut expansion
        let key = next_account(&mut iter)?;
        let owning_profile = next_account(&mut iter)?;
        let owning_profile_faction = next_account(&mut iter)?;
        let fleet = next_account(&mut iter)?;
        let game_id = next_account(&mut iter)?;
        let game_state = next_account(&mut iter)?;
        // Direct accounts
        let cargo_pod = next_account(&mut iter)?;
        let cargo_type = next_account(&mut iter)?;
        let cargo_stats_definition = next_account(&mut iter)?;
        let token = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;
        let funds_to = next_account(&mut iter)?;
        let cargo_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(CloseFleetCargoPodTokenAccountInstructionAccounts {
            key,
            owning_profile,
            owning_profile_faction,
            fleet,
            game_id,
            game_state,
            cargo_pod,
            cargo_type,
            cargo_stats_definition,
            token,
            token_mint,
            funds_to,
            cargo_program,
            token_program,
        })
    }
}
