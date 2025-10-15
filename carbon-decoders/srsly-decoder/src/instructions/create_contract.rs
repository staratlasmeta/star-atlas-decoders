use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf430f4b2d8587a34")]
pub struct CreateContract {
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

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let mint = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let owner_token_account = next_account(&mut iter)?;
        let fleet = next_account(&mut iter)?;
        let owner_profile = next_account(&mut iter)?;
        let game_id = next_account(&mut iter)?;
        let contract = next_account(&mut iter)?;
        let rental_authority = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let sage_program = next_account(&mut iter)?;

        Some(CreateContractInstructionAccounts {
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
        })
    }
}
