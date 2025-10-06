use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6921248aa5b53339")]
pub struct RegisterStarbase {
    pub input: RegisterStarbaseInputUnpacked,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RegisterStarbaseInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub starbase: solana_pubkey::Pubkey,
    pub sector: solana_pubkey::Pubkey,
    pub game_state_and_profile: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RegisterStarbase {
    type ArrangedAccounts = RegisterStarbaseInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funder = next_account(&mut iter)?;
        let starbase = next_account(&mut iter)?;
        let sector = next_account(&mut iter)?;
        let game_state_and_profile = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RegisterStarbaseInstructionAccounts {
            funder,
            starbase,
            sector,
            game_state_and_profile,
            system_program,
        })
    }
}
