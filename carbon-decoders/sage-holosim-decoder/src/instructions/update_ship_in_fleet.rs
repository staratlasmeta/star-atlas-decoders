use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd553bae5b31facfd")]
pub struct UpdateShipInFleet {
    pub input: UpdateShipFleetInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateShipInFleetInstructionAccounts {
    pub fleet: solana_pubkey::Pubkey,
    pub fleet_ships: solana_pubkey::Pubkey,
    pub old_ship: solana_pubkey::Pubkey,
    pub next: solana_pubkey::Pubkey,
    pub game_accounts: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateShipInFleet {
    type ArrangedAccounts = UpdateShipInFleetInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let fleet = next_account(&mut iter)?;
        let fleet_ships = next_account(&mut iter)?;
        let old_ship = next_account(&mut iter)?;
        let next = next_account(&mut iter)?;
        let game_accounts = next_account(&mut iter)?;

        Some(UpdateShipInFleetInstructionAccounts {
            fleet,
            fleet_ships,
            old_ship,
            next,
            game_accounts,
        })
    }
}
