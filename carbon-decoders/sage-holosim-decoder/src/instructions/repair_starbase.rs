use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8b603afb249fa34e")]
pub struct RepairStarbase {
    pub input: RepairStarbaseInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RepairStarbaseInstructionAccounts {
    pub game_and_fleet_and_owner: solana_pubkey::Pubkey,
    pub game_state: solana_pubkey::Pubkey,
    pub sage_player_profile: solana_pubkey::Pubkey,
    pub profile_faction: solana_pubkey::Pubkey,
    pub starbase: solana_pubkey::Pubkey,
    pub cargo_hold: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub stats_definition: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RepairStarbase {
    type ArrangedAccounts = RepairStarbaseInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_and_fleet_and_owner = next_account(&mut iter)?;
        let game_state = next_account(&mut iter)?;
        let sage_player_profile = next_account(&mut iter)?;
        let profile_faction = next_account(&mut iter)?;
        let starbase = next_account(&mut iter)?;
        let cargo_hold = next_account(&mut iter)?;
        let token_from = next_account(&mut iter)?;
        let cargo_type = next_account(&mut iter)?;
        let stats_definition = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;
        let cargo_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(RepairStarbaseInstructionAccounts {
            game_and_fleet_and_owner,
            game_state,
            sage_player_profile,
            profile_faction,
            starbase,
            cargo_hold,
            token_from,
            cargo_type,
            stats_definition,
            token_mint,
            cargo_program,
            token_program,
        })
    }
}
