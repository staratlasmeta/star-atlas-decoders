use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbad7501eaee2d321")]
pub struct StartMiningAsteroid {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct StartMiningAsteroidInstructionAccounts {
    pub game_accounts_fleet_and_owner: solana_pubkey::Pubkey,
    pub fleet_fuel_token_account: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub mine_item: solana_pubkey::Pubkey,
    pub resource: solana_pubkey::Pubkey,
    pub planet: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for StartMiningAsteroid {
    type ArrangedAccounts = StartMiningAsteroidInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_accounts_fleet_and_owner = next_account(&mut iter)?;
        let fleet_fuel_token_account = next_account(&mut iter)?;
        let starbase_and_starbase_player = next_account(&mut iter)?;
        let mine_item = next_account(&mut iter)?;
        let resource = next_account(&mut iter)?;
        let planet = next_account(&mut iter)?;

        Some(StartMiningAsteroidInstructionAccounts {
            game_accounts_fleet_and_owner,
            fleet_fuel_token_account,
            starbase_and_starbase_player,
            mine_item,
            resource,
            planet,
        })
    }
}
