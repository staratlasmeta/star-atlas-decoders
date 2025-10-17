use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd8e9a45c3addabed")]
pub struct TransferCargo {
    pub cargo_amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TransferCargoInstructionAccounts {
    pub origin_pod_authority: solana_pubkey::Pubkey,
    pub destination_pod_authority: solana_pubkey::Pubkey,
    pub stats_definition: solana_pubkey::Pubkey,
    pub origin_cargo_pod: solana_pubkey::Pubkey,
    pub destination_cargo_pod: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub origin_token_account: solana_pubkey::Pubkey,
    pub destination_token_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferCargo {
    type ArrangedAccounts = TransferCargoInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let origin_pod_authority = next_account(&mut iter)?;
        let destination_pod_authority = next_account(&mut iter)?;
        let stats_definition = next_account(&mut iter)?;
        let origin_cargo_pod = next_account(&mut iter)?;
        let destination_cargo_pod = next_account(&mut iter)?;
        let cargo_type = next_account(&mut iter)?;
        let origin_token_account = next_account(&mut iter)?;
        let destination_token_account = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(TransferCargoInstructionAccounts {
            origin_pod_authority,
            destination_pod_authority,
            stats_definition,
            origin_cargo_pod,
            destination_cargo_pod,
            cargo_type,
            origin_token_account,
            destination_token_account,
            token_program,
        })
    }
}
