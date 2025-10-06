use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd55b4e76cf8562ee")]
pub struct RegisterPlanet {
    pub input: RegisterPlanetInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RegisterPlanetInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub planet: solana_pubkey::Pubkey,
    pub sector: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RegisterPlanet {
    type ArrangedAccounts = RegisterPlanetInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_and_profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let planet = next_account(&mut iter)?;
        let sector = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RegisterPlanetInstructionAccounts {
            game_and_profile,
            funder,
            planet,
            sector,
            system_program,
        })
    }
}
