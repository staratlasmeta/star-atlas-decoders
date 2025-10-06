use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4c7b5152ebe49ccb")]
pub struct CreateFleet {
    pub input: CreateFleetInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateFleetInstructionAccounts {
    pub game_accounts_and_profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub fleet: solana_pubkey::Pubkey,
    pub fleet_ships: solana_pubkey::Pubkey,
    pub cargo_hold: solana_pubkey::Pubkey,
    pub fuel_tank: solana_pubkey::Pubkey,
    pub ammo_bank: solana_pubkey::Pubkey,
    pub ship: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateFleet {
    type ArrangedAccounts = CreateFleetInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_accounts_and_profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let fleet = next_account(&mut iter)?;
        let fleet_ships = next_account(&mut iter)?;
        let cargo_hold = next_account(&mut iter)?;
        let fuel_tank = next_account(&mut iter)?;
        let ammo_bank = next_account(&mut iter)?;
        let ship = next_account(&mut iter)?;
        let starbase_and_starbase_player = next_account(&mut iter)?;
        let cargo_stats_definition = next_account(&mut iter)?;
        let cargo_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CreateFleetInstructionAccounts {
            game_accounts_and_profile,
            funder,
            fleet,
            fleet_ships,
            cargo_hold,
            fuel_tank,
            ammo_bank,
            ship,
            starbase_and_starbase_player,
            cargo_stats_definition,
            cargo_program,
            system_program,
        })
    }
}
