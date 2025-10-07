use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd03f876cdafc2400")]
pub struct MineAsteroidToRespawn {
    pub input: MineAsteroidToRespawnInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MineAsteroidToRespawnInstructionAccounts {
    // GameAndGameStateAndFleetAndOwnerMut expansion
    pub key: solana_pubkey::Pubkey,
    pub owning_profile: solana_pubkey::Pubkey,
    pub owning_profile_faction: solana_pubkey::Pubkey,
    pub fleet: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub game_state: solana_pubkey::Pubkey,
    pub resource: solana_pubkey::Pubkey,
    pub planet: solana_pubkey::Pubkey,
    pub atlas_token_from: solana_pubkey::Pubkey,
    pub atlas_token_to: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MineAsteroidToRespawn {
    type ArrangedAccounts = MineAsteroidToRespawnInstructionAccounts;

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

        let resource = next_account(&mut iter)?;
        let planet = next_account(&mut iter)?;
        let atlas_token_from = next_account(&mut iter)?;
        let atlas_token_to = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(MineAsteroidToRespawnInstructionAccounts {
            key,
            owning_profile,
            owning_profile_faction,
            fleet,
            game_id,
            game_state,
            resource,
            planet,
            atlas_token_from,
            atlas_token_to,
            token_program,
        })
    }
}
