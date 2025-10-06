use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x27d99652edd664b8")]
pub struct UpdateProgressionConfig {
    pub input: UpdateProgressionConfigInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateProgressionConfigInstructionAccounts {
    pub progression_config: solana_pubkey::Pubkey,
    pub game_and_profile: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateProgressionConfig {
    type ArrangedAccounts = UpdateProgressionConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let progression_config = next_account(&mut iter)?;
        let game_and_profile = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(UpdateProgressionConfigInstructionAccounts {
            progression_config,
            game_and_profile,
            system_program,
        })
    }
}
