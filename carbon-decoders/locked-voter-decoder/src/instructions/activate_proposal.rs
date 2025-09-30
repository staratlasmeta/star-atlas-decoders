use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5abacbea46b9bf15")]
pub struct ActivateProposal {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ActivateProposalInstructionAccounts {
    pub locker: solana_pubkey::Pubkey,
    pub governor: solana_pubkey::Pubkey,
    pub proposal: solana_pubkey::Pubkey,
    pub escrow: solana_pubkey::Pubkey,
    pub escrow_owner: solana_pubkey::Pubkey,
    pub govern_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ActivateProposal {
    type ArrangedAccounts = ActivateProposalInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let locker = next_account(&mut iter)?;
        let governor = next_account(&mut iter)?;
        let proposal = next_account(&mut iter)?;
        let escrow = next_account(&mut iter)?;
        let escrow_owner = next_account(&mut iter)?;
        let govern_program = next_account(&mut iter)?;

        Some(ActivateProposalInstructionAccounts {
            locker,
            governor,
            proposal,
            escrow,
            escrow_owner,
            govern_program,
        })
    }
}
