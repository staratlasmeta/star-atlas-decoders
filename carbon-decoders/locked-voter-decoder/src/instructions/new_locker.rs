use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb185205ae5d8832f")]
pub struct NewLocker {
    pub bump: u8,
    pub params: LockerParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct NewLockerInstructionAccounts {
    pub base: solana_pubkey::Pubkey,
    pub locker: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub governor: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for NewLocker {
    type ArrangedAccounts = NewLockerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let base = next_account(&mut iter)?;
        let locker = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;
        let governor = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(NewLockerInstructionAccounts {
            base,
            locker,
            token_mint,
            governor,
            payer,
            system_program,
        })
    }
}
