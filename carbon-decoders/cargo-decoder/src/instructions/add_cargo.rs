use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbcdc9f6c814961bb")]
pub struct AddCargo {
    pub cargo_amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AddCargoInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub signer_origin_account: solana_pubkey::Pubkey,
    pub stats_definition: solana_pubkey::Pubkey,
    pub cargo_pod: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub origin_token_account: solana_pubkey::Pubkey,
    pub cargo_token_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddCargo {
    type ArrangedAccounts = AddCargoInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let authority = next_account(&mut iter)?;
        let signer_origin_account = next_account(&mut iter)?;
        let stats_definition = next_account(&mut iter)?;
        let cargo_pod = next_account(&mut iter)?;
        let cargo_type = next_account(&mut iter)?;
        let origin_token_account = next_account(&mut iter)?;
        let cargo_token_account = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(AddCargoInstructionAccounts {
            authority,
            signer_origin_account,
            stats_definition,
            cargo_pod,
            cargo_type,
            origin_token_account,
            cargo_token_account,
            token_program,
        })
    }
}
