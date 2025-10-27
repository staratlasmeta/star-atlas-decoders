use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x726e24cfbc1d6ffb")]
pub struct RemoveCargo {
    pub cargo_amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RemoveCargoInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub stats_definition: solana_pubkey::Pubkey,
    pub cargo_pod: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub destination_token_account: solana_pubkey::Pubkey,
    pub cargo_token_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveCargo {
    type ArrangedAccounts = RemoveCargoInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let authority = next_account(&mut iter)?;
        let stats_definition = next_account(&mut iter)?;
        let cargo_pod = next_account(&mut iter)?;
        let cargo_type = next_account(&mut iter)?;
        let destination_token_account = next_account(&mut iter)?;
        let cargo_token_account = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(RemoveCargoInstructionAccounts {
            authority,
            stats_definition,
            cargo_pod,
            cargo_type,
            destination_token_account,
            cargo_token_account,
            token_program,
        })
    }
}
