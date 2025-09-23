use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x06337f5c394cb0a5")]
pub struct RegisterShip {
    pub input: RegisterShipInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RegisterShipInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub ship: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RegisterShip {
    type ArrangedAccounts = RegisterShipInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_and_profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let ship = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RegisterShipInstructionAccounts {
            game_and_profile,
            funder,
            ship,
            mint,
            system_program,
        })
    }
}
