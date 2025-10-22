use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb2e844d95970caaf")]
pub struct ChooseFaction {
    pub key_index: u16,
    pub faction: Faction,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ChooseFactionInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub faction: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ChooseFaction {
    type ArrangedAccounts = ChooseFactionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let faction = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(ChooseFactionInstructionAccounts {
            key,
            funder,
            profile,
            faction,
            system_program,
        })
    }
}
