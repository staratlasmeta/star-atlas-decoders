use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x573c329af16a4d17")]
pub struct WarpToCoordinate {
    pub input: WarpToCoordinateInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct WarpToCoordinateInstructionAccounts {
    pub game_accounts_fleet_and_owner: solana_pubkey::Pubkey,
    pub fuel_tank: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub stats_definition: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WarpToCoordinate {
    type ArrangedAccounts = WarpToCoordinateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_accounts_fleet_and_owner = next_account(&mut iter)?;
        let fuel_tank = next_account(&mut iter)?;
        let cargo_type = next_account(&mut iter)?;
        let stats_definition = next_account(&mut iter)?;
        let token_from = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;
        let cargo_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(WarpToCoordinateInstructionAccounts {
            game_accounts_fleet_and_owner,
            fuel_tank,
            cargo_type,
            stats_definition,
            token_from,
            token_mint,
            cargo_program,
            token_program,
        })
    }
}
