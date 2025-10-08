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
    // GameAndGameStateAndFleetAndOwnerMut expansion
    pub key: solana_pubkey::Pubkey,
    pub owning_profile: solana_pubkey::Pubkey,
    pub owning_profile_faction: solana_pubkey::Pubkey,
    pub fleet: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub game_state: solana_pubkey::Pubkey,
    pub fleet_fuel_token_account: solana_pubkey::Pubkey,
    // StarbaseMutAndStarbasePlayer expansion
    pub starbase: solana_pubkey::Pubkey,
    pub starbase_player: solana_pubkey::Pubkey,
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

        // GameAndGameStateAndFleetAndOwnerMut expansion
        let key = next_account(&mut iter)?;
        let owning_profile = next_account(&mut iter)?;
        let owning_profile_faction = next_account(&mut iter)?;
        let fleet = next_account(&mut iter)?;
        let game_id = next_account(&mut iter)?;
        let game_state = next_account(&mut iter)?;

        let fleet_fuel_token_account = next_account(&mut iter)?;

        // StarbaseMutAndStarbasePlayer expansion
        let starbase = next_account(&mut iter)?;
        let starbase_player = next_account(&mut iter)?;

        let mine_item = next_account(&mut iter)?;
        let resource = next_account(&mut iter)?;
        let planet = next_account(&mut iter)?;

        Some(StartMiningAsteroidInstructionAccounts {
            key,
            owning_profile,
            owning_profile_faction,
            fleet,
            game_id,
            game_state,
            fleet_fuel_token_account,
            starbase,
            starbase_player,
            mine_item,
            resource,
            planet,
        })
    }
}
