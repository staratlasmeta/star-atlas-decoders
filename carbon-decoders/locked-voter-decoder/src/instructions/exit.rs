use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xea200c477e05dba0")]
pub struct Exit {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ExitInstructionAccounts {
    pub locker: solana_pubkey::Pubkey,
    pub escrow: solana_pubkey::Pubkey,
    pub escrow_owner: solana_pubkey::Pubkey,
    pub escrow_tokens: solana_pubkey::Pubkey,
    pub destination_tokens: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Exit {
    type ArrangedAccounts = ExitInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let locker = next_account(&mut iter)?;
        let escrow = next_account(&mut iter)?;
        let escrow_owner = next_account(&mut iter)?;
        let escrow_tokens = next_account(&mut iter)?;
        let destination_tokens = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(ExitInstructionAccounts {
            locker,
            escrow,
            escrow_owner,
            escrow_tokens,
            destination_tokens,
            payer,
            token_program,
        })
    }
}
