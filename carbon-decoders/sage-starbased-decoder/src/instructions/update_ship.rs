use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x67e724d5bb482afc")]
pub struct UpdateShip {
    pub input: UpdateShipInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateShipInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub ship: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateShip {
    type ArrangedAccounts = UpdateShipInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_and_profile = next_account(&mut iter)?;
        let ship = next_account(&mut iter)?;

        Some(UpdateShipInstructionAccounts {
            game_and_profile,
            ship,
        })
    }
}
