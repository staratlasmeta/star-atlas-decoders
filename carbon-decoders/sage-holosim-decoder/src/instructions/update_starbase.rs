use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4e9268a66f925f1c")]
pub struct UpdateStarbase {
    pub input: UpdateStarbaseInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateStarbaseInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub starbase: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateStarbase {
    type ArrangedAccounts = UpdateStarbaseInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_and_profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let starbase = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(UpdateStarbaseInstructionAccounts {
            game_and_profile,
            funder,
            starbase,
            system_program,
        })
    }
}
