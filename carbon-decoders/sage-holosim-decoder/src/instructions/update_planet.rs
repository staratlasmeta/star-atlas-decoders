use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x182c47861f20c9b2")]
pub struct UpdatePlanet {
    pub input: UpdatePlanetInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdatePlanetInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub planet: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePlanet {
    type ArrangedAccounts = UpdatePlanetInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_and_profile = next_account(&mut iter)?;
        let planet = next_account(&mut iter)?;

        Some(UpdatePlanetInstructionAccounts {
            game_and_profile,
            planet,
        })
    }
}
