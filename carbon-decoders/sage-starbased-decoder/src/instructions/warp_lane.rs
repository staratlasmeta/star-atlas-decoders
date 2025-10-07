use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe8bcc3316448e7f3")]
pub struct WarpLane {
    pub input: WarpLaneInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct WarpLaneInstructionAccounts {
    // GameAndGameStateAndFleetAndOwnerMut expansion
    pub key: solana_pubkey::Pubkey,
    pub owning_profile: solana_pubkey::Pubkey,
    pub owning_profile_faction: solana_pubkey::Pubkey,
    pub fleet: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub game_state: solana_pubkey::Pubkey,
    // Direct accounts
    pub from_starbase: solana_pubkey::Pubkey,
    pub to_starbase: solana_pubkey::Pubkey,
    pub from_sector: solana_pubkey::Pubkey,
    pub to_sector: solana_pubkey::Pubkey,
    pub fuel_tank: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub stats_definition: solana_pubkey::Pubkey,
    pub fuel_token_from: solana_pubkey::Pubkey,
    pub fuel_mint: solana_pubkey::Pubkey,
    pub fee_token_from: solana_pubkey::Pubkey,
    pub fee_token_to: solana_pubkey::Pubkey,
    pub fee_mint: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WarpLane {
    type ArrangedAccounts = WarpLaneInstructionAccounts;

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
        let from_starbase = next_account(&mut iter)?;
        let to_starbase = next_account(&mut iter)?;
        let from_sector = next_account(&mut iter)?;
        let to_sector = next_account(&mut iter)?;
        let fuel_tank = next_account(&mut iter)?;
        let cargo_type = next_account(&mut iter)?;
        let stats_definition = next_account(&mut iter)?;
        let fuel_token_from = next_account(&mut iter)?;
        let fuel_mint = next_account(&mut iter)?;
        let fee_token_from = next_account(&mut iter)?;
        let fee_token_to = next_account(&mut iter)?;
        let fee_mint = next_account(&mut iter)?;
        let cargo_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(WarpLaneInstructionAccounts {
            key,
            owning_profile,
            owning_profile_faction,
            fleet,
            game_id,
            game_state,
            from_starbase,
            to_starbase,
            from_sector,
            to_sector,
            fuel_tank,
            cargo_type,
            stats_definition,
            fuel_token_from,
            fuel_mint,
            fee_token_from,
            fee_token_to,
            fee_mint,
            cargo_program,
            token_program,
        })
    }
}
