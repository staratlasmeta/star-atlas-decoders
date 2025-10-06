use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa55a47de9b070e79")]
pub struct DisbandedFleetToEscrow {
    pub input: DisbandedFleetToEscrowInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DisbandedFleetToEscrowInstructionAccounts {
    pub game_accounts_and_profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub disbanded_fleet: solana_pubkey::Pubkey,
    pub fleet_ships: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub ship: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DisbandedFleetToEscrow {
    type ArrangedAccounts = DisbandedFleetToEscrowInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_accounts_and_profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let disbanded_fleet = next_account(&mut iter)?;
        let fleet_ships = next_account(&mut iter)?;
        let starbase_and_starbase_player = next_account(&mut iter)?;
        let ship = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(DisbandedFleetToEscrowInstructionAccounts {
            game_accounts_and_profile,
            funder,
            disbanded_fleet,
            fleet_ships,
            starbase_and_starbase_player,
            ship,
            system_program,
        })
    }
}
