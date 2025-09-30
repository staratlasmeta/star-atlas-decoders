use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd8b68f0bdc2656b9")]
pub struct NewEscrow {
    pub bump: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct NewEscrowInstructionAccounts {
    pub locker: solana_pubkey::Pubkey,
    pub escrow: solana_pubkey::Pubkey,
    pub escrow_owner: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for NewEscrow {
    type ArrangedAccounts = NewEscrowInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let locker = next_account(&mut iter)?;
        let escrow = next_account(&mut iter)?;
        let escrow_owner = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(NewEscrowInstructionAccounts {
            locker,
            escrow,
            escrow_owner,
            payer,
            system_program,
        })
    }
}
