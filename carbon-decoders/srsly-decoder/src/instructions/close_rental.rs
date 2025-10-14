

use carbon_core::{CarbonDeserialize, borsh, account_utils::next_account};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xb3bc71d329e83333")]
pub struct CloseRental{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseRentalInstructionAccounts {
    pub borrower: solana_pubkey::Pubkey,
    pub borrower_token_account: solana_pubkey::Pubkey,
    pub owner_token_account: solana_pubkey::Pubkey,
    pub contract: solana_pubkey::Pubkey,
    pub rental_state: solana_pubkey::Pubkey,
    pub rental_token_account: solana_pubkey::Pubkey,
    pub rental_authority: solana_pubkey::Pubkey,
    pub rental_thread: solana_pubkey::Pubkey,
    pub antegen_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseRental {
    type ArrangedAccounts = CloseRentalInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let borrower = next_account(&mut iter)?;
        let borrower_token_account = next_account(&mut iter)?;
        let owner_token_account = next_account(&mut iter)?;
        let contract = next_account(&mut iter)?;
        let rental_state = next_account(&mut iter)?;
        let rental_token_account = next_account(&mut iter)?;
        let rental_authority = next_account(&mut iter)?;
        let rental_thread = next_account(&mut iter)?;
        let antegen_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CloseRentalInstructionAccounts {
            borrower,
            borrower_token_account,
            owner_token_account,
            contract,
            rental_state,
            rental_token_account,
            rental_authority,
            rental_thread,
            antegen_program,
            token_program,
            system_program,
        })
    }
}