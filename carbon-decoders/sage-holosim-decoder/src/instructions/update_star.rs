use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x91920c560c3027a9")]
pub struct UpdateStar {
    pub input: UpdateStarInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateStarInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub star: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateStar {
    type ArrangedAccounts = UpdateStarInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_and_profile = next_account(&mut iter)?;
        let star = next_account(&mut iter)?;

        Some(UpdateStarInstructionAccounts {
            game_and_profile,
            star,
        })
    }
}
