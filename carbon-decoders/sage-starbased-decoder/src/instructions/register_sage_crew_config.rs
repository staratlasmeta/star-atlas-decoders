use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdd907d9e1c19bad1")]
pub struct RegisterSageCrewConfig {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RegisterSageCrewConfigInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub sage_crew_config: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub game_and_profile: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RegisterSageCrewConfig {
    type ArrangedAccounts = RegisterSageCrewConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funder = next_account(&mut iter)?;
        let sage_crew_config = next_account(&mut iter)?;
        let config = next_account(&mut iter)?;
        let game_and_profile = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RegisterSageCrewConfigInstructionAccounts {
            funder,
            sage_crew_config,
            config,
            game_and_profile,
            system_program,
        })
    }
}
