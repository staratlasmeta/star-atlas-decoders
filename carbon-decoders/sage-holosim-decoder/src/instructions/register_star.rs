use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0f1969eab1d793bf")]
pub struct RegisterStar {
    pub input: RegisterStarInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RegisterStarInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub star: solana_pubkey::Pubkey,
    pub sector: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RegisterStar {
    type ArrangedAccounts = RegisterStarInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_and_profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let star = next_account(&mut iter)?;
        let sector = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RegisterStarInstructionAccounts {
            game_and_profile,
            funder,
            star,
            sector,
            system_program,
        })
    }
}
