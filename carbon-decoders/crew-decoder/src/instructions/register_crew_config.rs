use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfd1c8664fd2e4ee5")]
pub struct RegisterCrewConfig {
    pub args: RegisterCrewConfigArgs,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RegisterCrewConfigInstructionAccounts {
    pub profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub crew_config: solana_pubkey::Pubkey,
    pub seed_pubkey: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RegisterCrewConfig {
    type ArrangedAccounts = RegisterCrewConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let crew_config = next_account(&mut iter)?;
        let seed_pubkey = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RegisterCrewConfigInstructionAccounts {
            profile,
            funder,
            crew_config,
            seed_pubkey,
            system_program,
        })
    }
}
