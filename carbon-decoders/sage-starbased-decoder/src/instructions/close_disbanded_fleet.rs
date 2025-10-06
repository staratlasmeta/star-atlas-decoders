use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd696959cf57b25a5")]
pub struct CloseDisbandedFleet {
    pub input: CloseDisbandedFleetInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseDisbandedFleetInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub player_profile: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub disbanded_fleet: solana_pubkey::Pubkey,
    pub fleet_ships: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseDisbandedFleet {
    type ArrangedAccounts = CloseDisbandedFleetInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let player_profile = next_account(&mut iter)?;
        let funds_to = next_account(&mut iter)?;
        let disbanded_fleet = next_account(&mut iter)?;
        let fleet_ships = next_account(&mut iter)?;

        Some(CloseDisbandedFleetInstructionAccounts {
            key,
            player_profile,
            funds_to,
            disbanded_fleet,
            fleet_ships,
        })
    }
}
