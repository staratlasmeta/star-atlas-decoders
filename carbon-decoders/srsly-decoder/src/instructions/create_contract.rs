

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xf430f4b2d8587a34")]
pub struct CreateContract{
    pub rate: u64,
    pub duration_min: u64,
    pub duration_max: u64,
    pub payments_feq: String,
    pub owner_key_index: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateContractInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub owner_token_account: solana_pubkey::Pubkey,
    pub fleet: solana_pubkey::Pubkey,
    pub owner_profile: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub contract: solana_pubkey::Pubkey,
    pub rental_authority: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub sage_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateContract {
    type ArrangedAccounts = CreateContractInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            mint,
            owner,
            owner_token_account,
            fleet,
            owner_profile,
            game_id,
            contract,
            rental_authority,
            token_program,
            associated_token_program,
            system_program,
            sage_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(CreateContractInstructionAccounts {
            mint: mint.pubkey,
            owner: owner.pubkey,
            owner_token_account: owner_token_account.pubkey,
            fleet: fleet.pubkey,
            owner_profile: owner_profile.pubkey,
            game_id: game_id.pubkey,
            contract: contract.pubkey,
            rental_authority: rental_authority.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            sage_program: sage_program.pubkey,
        })
    }
}