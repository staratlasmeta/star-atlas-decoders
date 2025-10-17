use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x09f6e88937f23766")]
pub struct DrainCraftableItemBank {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DrainCraftableItemBankInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub craftable_item: solana_pubkey::Pubkey,
    pub domain: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub token_to: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DrainCraftableItemBank {
    type ArrangedAccounts = DrainCraftableItemBankInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let funds_to = next_account(&mut iter)?;
        let craftable_item = next_account(&mut iter)?;
        let domain = next_account(&mut iter)?;
        let token_from = next_account(&mut iter)?;
        let token_to = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(DrainCraftableItemBankInstructionAccounts {
            key,
            profile,
            funds_to,
            craftable_item,
            domain,
            token_from,
            token_to,
            token_program,
        })
    }
}
