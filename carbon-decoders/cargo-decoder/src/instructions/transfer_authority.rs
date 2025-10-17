use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x30a94c48e5b437a1")]
pub struct TransferAuthority {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TransferAuthorityInstructionAccounts {
    pub origin_pod_authority: solana_pubkey::Pubkey,
    pub new_pod_authority: solana_pubkey::Pubkey,
    pub cargo_pod: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferAuthority {
    type ArrangedAccounts = TransferAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let origin_pod_authority = next_account(&mut iter)?;
        let new_pod_authority = next_account(&mut iter)?;
        let cargo_pod = next_account(&mut iter)?;

        Some(TransferAuthorityInstructionAccounts {
            origin_pod_authority,
            new_pod_authority,
            cargo_pod,
        })
    }
}
