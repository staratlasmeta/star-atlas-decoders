use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x720f6fcf73cf6ca9")]
pub struct PayRental {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct PayRentalInstructionAccounts {
    pub borrower: solana_pubkey::Pubkey,
    pub borrower_token_account: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub owner_token_account: solana_pubkey::Pubkey,
    pub fleet: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub starbase: solana_pubkey::Pubkey,
    pub starbase_player: solana_pubkey::Pubkey,
    pub contract: solana_pubkey::Pubkey,
    pub rental_state: solana_pubkey::Pubkey,
    pub rental_authority: solana_pubkey::Pubkey,
    pub rental_token_account: solana_pubkey::Pubkey,
    pub rental_thread: solana_pubkey::Pubkey,
    pub sage_program: solana_pubkey::Pubkey,
    pub antegen_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PayRental {
    type ArrangedAccounts = PayRentalInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let borrower = next_account(&mut iter)?;
        let borrower_token_account = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let owner_token_account = next_account(&mut iter)?;
        let fleet = next_account(&mut iter)?;
        let game_id = next_account(&mut iter)?;
        let starbase = next_account(&mut iter)?;
        let starbase_player = next_account(&mut iter)?;
        let contract = next_account(&mut iter)?;
        let rental_state = next_account(&mut iter)?;
        let rental_authority = next_account(&mut iter)?;
        let rental_token_account = next_account(&mut iter)?;
        let rental_thread = next_account(&mut iter)?;
        let sage_program = next_account(&mut iter)?;
        let antegen_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(PayRentalInstructionAccounts {
            borrower,
            borrower_token_account,
            owner,
            owner_token_account,
            fleet,
            game_id,
            starbase,
            starbase_player,
            contract,
            rental_state,
            rental_authority,
            rental_token_account,
            rental_thread,
            sage_program,
            antegen_program,
            token_program,
        })
    }
}
