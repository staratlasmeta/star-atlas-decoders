use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x57d1a41c0652e8d6")]
pub struct RegisterResource {
    pub input: RegisterResourceInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RegisterResourceInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub resource: solana_pubkey::Pubkey,
    pub location: solana_pubkey::Pubkey,
    pub mine_item: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RegisterResource {
    type ArrangedAccounts = RegisterResourceInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_and_profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let resource = next_account(&mut iter)?;
        let location = next_account(&mut iter)?;
        let mine_item = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RegisterResourceInstructionAccounts {
            game_and_profile,
            funder,
            resource,
            location,
            mine_item,
            system_program,
        })
    }
}
