

use carbon_core::{CarbonDeserialize, borsh, account_utils::next_account};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x25f422a85cca506a")]
pub struct CloseContract{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseContractInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub owner_token_account: solana_pubkey::Pubkey,
    pub rental_token_account: solana_pubkey::Pubkey,
    pub borrower_token_account: solana_pubkey::Pubkey,
    pub rental_state: solana_pubkey::Pubkey,
    pub fleet: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub starbase: solana_pubkey::Pubkey,
    pub starbase_player: solana_pubkey::Pubkey,
    pub contract: solana_pubkey::Pubkey,
    pub rental_authority: solana_pubkey::Pubkey,
    pub sage_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseContract {
    type ArrangedAccounts = CloseContractInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let owner = next_account(&mut iter)?;
        let owner_token_account = next_account(&mut iter)?;
        let rental_token_account = next_account(&mut iter)?;
        let borrower_token_account = next_account(&mut iter)?;
        let rental_state = next_account(&mut iter)?;
        let fleet = next_account(&mut iter)?;
        let game_id = next_account(&mut iter)?;
        let starbase = next_account(&mut iter)?;
        let starbase_player = next_account(&mut iter)?;
        let contract = next_account(&mut iter)?;
        let rental_authority = next_account(&mut iter)?;
        let sage_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(CloseContractInstructionAccounts {
            owner,
            owner_token_account,
            rental_token_account,
            borrower_token_account,
            rental_state,
            fleet,
            game_id,
            starbase,
            starbase_player,
            contract,
            rental_authority,
            sage_program,
            token_program,
        })
    }
}