use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1513d02bed3eff57")]
pub struct Lock {
    pub amount: u64,
    pub duration: i64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct LockInstructionAccounts {
    pub locker: solana_pubkey::Pubkey,
    pub escrow: solana_pubkey::Pubkey,
    pub escrow_tokens: solana_pubkey::Pubkey,
    pub escrow_owner: solana_pubkey::Pubkey,
    pub source_tokens: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Lock {
    type ArrangedAccounts = LockInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let locker = next_account(&mut iter)?;
        let escrow = next_account(&mut iter)?;
        let escrow_tokens = next_account(&mut iter)?;
        let escrow_owner = next_account(&mut iter)?;
        let source_tokens = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(LockInstructionAccounts {
            locker,
            escrow,
            escrow_tokens,
            escrow_owner,
            source_tokens,
            token_program,
        })
    }
}
