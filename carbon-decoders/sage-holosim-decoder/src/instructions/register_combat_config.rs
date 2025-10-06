use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfb65fbcda39b7979")]
pub struct RegisterCombatConfig {
    pub input: RegisterCombatConfigInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RegisterCombatConfigInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub combat_config: solana_pubkey::Pubkey,
    pub game_and_profile: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RegisterCombatConfig {
    type ArrangedAccounts = RegisterCombatConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funder = next_account(&mut iter)?;
        let combat_config = next_account(&mut iter)?;
        let game_and_profile = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RegisterCombatConfigInstructionAccounts {
            funder,
            combat_config,
            game_and_profile,
            system_program,
        })
    }
}
