use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x12f535779b39014e")]
pub struct ForceDisbandFleet {
    pub input: ForcedDisbandFleetInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ForceDisbandFleetInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub disbanded_fleet: solana_pubkey::Pubkey,
    pub fleet: solana_pubkey::Pubkey,
    pub fleet_ships: solana_pubkey::Pubkey,
    pub cargo_hold: solana_pubkey::Pubkey,
    pub fuel_tank: solana_pubkey::Pubkey,
    pub ammo_bank: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub ship: solana_pubkey::Pubkey,
    pub game_accounts: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ForceDisbandFleet {
    type ArrangedAccounts = ForceDisbandFleetInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funder = next_account(&mut iter)?;
        let disbanded_fleet = next_account(&mut iter)?;
        let fleet = next_account(&mut iter)?;
        let fleet_ships = next_account(&mut iter)?;
        let cargo_hold = next_account(&mut iter)?;
        let fuel_tank = next_account(&mut iter)?;
        let ammo_bank = next_account(&mut iter)?;
        let starbase_and_starbase_player = next_account(&mut iter)?;
        let ship = next_account(&mut iter)?;
        let game_accounts = next_account(&mut iter)?;
        let cargo_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(ForceDisbandFleetInstructionAccounts {
            funder,
            disbanded_fleet,
            fleet,
            fleet_ships,
            cargo_hold,
            fuel_tank,
            ammo_bank,
            starbase_and_starbase_player,
            ship,
            game_accounts,
            cargo_program,
            system_program,
        })
    }
}
