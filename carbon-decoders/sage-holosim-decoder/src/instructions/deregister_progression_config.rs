use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf9aef2bb8bdb2f76")]
pub struct DeregisterProgressionConfig {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DeregisterProgressionConfigInstructionAccounts {
    pub funds_to: solana_pubkey::Pubkey,
    pub progression_config: solana_pubkey::Pubkey,
    pub game_and_profile: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeregisterProgressionConfig {
    type ArrangedAccounts = DeregisterProgressionConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funds_to = next_account(&mut iter)?;
        let progression_config = next_account(&mut iter)?;
        let game_and_profile = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(DeregisterProgressionConfigInstructionAccounts {
            funds_to,
            progression_config,
            game_and_profile,
            system_program,
        })
    }
}
