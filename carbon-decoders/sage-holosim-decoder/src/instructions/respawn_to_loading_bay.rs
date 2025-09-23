use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x69c48bee0d467de2")]
pub struct RespawnToLoadingBay {
    pub input: RespawnToLoadingBayInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RespawnToLoadingBayInstructionAccounts {
    pub game_fleet_and_owner: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub cargo_hold: solana_pubkey::Pubkey,
    pub fuel_tank: solana_pubkey::Pubkey,
    pub ammo_bank: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RespawnToLoadingBay {
    type ArrangedAccounts = RespawnToLoadingBayInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_fleet_and_owner = next_account(&mut iter)?;
        let starbase_and_starbase_player = next_account(&mut iter)?;
        let cargo_hold = next_account(&mut iter)?;
        let fuel_tank = next_account(&mut iter)?;
        let ammo_bank = next_account(&mut iter)?;

        Some(RespawnToLoadingBayInstructionAccounts {
            game_fleet_and_owner,
            starbase_and_starbase_player,
            cargo_hold,
            fuel_tank,
            ammo_bank,
        })
    }
}
