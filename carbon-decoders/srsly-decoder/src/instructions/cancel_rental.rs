

use carbon_core::{CarbonDeserialize, borsh, account_utils::next_account};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x61cc3f0854221c2b")]
pub struct CancelRental{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CancelRentalInstructionAccounts {
    pub borrower: solana_pubkey::Pubkey,
    pub rental_thread: solana_pubkey::Pubkey,
    pub contract: solana_pubkey::Pubkey,
    pub rental_state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelRental {
    type ArrangedAccounts = CancelRentalInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let borrower = next_account(&mut iter)?;
        let rental_thread = next_account(&mut iter)?;
        let contract = next_account(&mut iter)?;
        let rental_state = next_account(&mut iter)?;

        Some(CancelRentalInstructionAccounts {
            borrower,
            rental_thread,
            contract,
            rental_state,
        })
    }
}