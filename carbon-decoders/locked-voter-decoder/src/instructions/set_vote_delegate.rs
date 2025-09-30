use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2eecf1f3fb6c9c0c")]
pub struct SetVoteDelegate {
    pub new_delegate: solana_pubkey::Pubkey,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetVoteDelegateInstructionAccounts {
    pub escrow: solana_pubkey::Pubkey,
    pub escrow_owner: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetVoteDelegate {
    type ArrangedAccounts = SetVoteDelegateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let escrow = next_account(&mut iter)?;
        let escrow_owner = next_account(&mut iter)?;

        Some(SetVoteDelegateInstructionAccounts {
            escrow,
            escrow_owner,
        })
    }
}
