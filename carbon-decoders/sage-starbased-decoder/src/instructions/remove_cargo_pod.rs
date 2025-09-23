use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd8174e68ef2b0803")]
pub struct RemoveCargoPod {
    pub input: StarbaseRemoveCargoPodInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RemoveCargoPodInstructionAccounts {
    pub funds_to: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub cargo_pod: solana_pubkey::Pubkey,
    pub game_accounts_and_profile: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveCargoPod {
    type ArrangedAccounts = RemoveCargoPodInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funds_to = next_account(&mut iter)?;
        let starbase_and_starbase_player = next_account(&mut iter)?;
        let cargo_pod = next_account(&mut iter)?;
        let game_accounts_and_profile = next_account(&mut iter)?;
        let cargo_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RemoveCargoPodInstructionAccounts {
            funds_to,
            starbase_and_starbase_player,
            cargo_pod,
            game_accounts_and_profile,
            cargo_program,
            system_program,
        })
    }
}
