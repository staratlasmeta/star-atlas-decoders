

use carbon_core::{CarbonDeserialize, borsh};


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
        let [
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
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(CloseContractInstructionAccounts {
            owner: owner.pubkey,
            owner_token_account: owner_token_account.pubkey,
            rental_token_account: rental_token_account.pubkey,
            borrower_token_account: borrower_token_account.pubkey,
            rental_state: rental_state.pubkey,
            fleet: fleet.pubkey,
            game_id: game_id.pubkey,
            starbase: starbase.pubkey,
            starbase_player: starbase_player.pubkey,
            contract: contract.pubkey,
            rental_authority: rental_authority.pubkey,
            sage_program: sage_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}