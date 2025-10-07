use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x14d40fbd45b44597")]
pub struct CastVote {
    pub side: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CastVoteInstructionAccounts {
    pub locker: solana_pubkey::Pubkey,
    pub escrow: solana_pubkey::Pubkey,
    pub vote_delegate: solana_pubkey::Pubkey,
    pub proposal: solana_pubkey::Pubkey,
    pub vote: solana_pubkey::Pubkey,
    pub governor: solana_pubkey::Pubkey,
    pub govern_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CastVote {
    type ArrangedAccounts = CastVoteInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let locker = next_account(&mut iter)?;
        let escrow = next_account(&mut iter)?;
        let vote_delegate = next_account(&mut iter)?;
        let proposal = next_account(&mut iter)?;
        let vote = next_account(&mut iter)?;
        let governor = next_account(&mut iter)?;
        let govern_program = next_account(&mut iter)?;

        Some(CastVoteInstructionAccounts {
            locker,
            escrow,
            vote_delegate,
            proposal,
            vote,
            governor,
            govern_program,
        })
    }
}
