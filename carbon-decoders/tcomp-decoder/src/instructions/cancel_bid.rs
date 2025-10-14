use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x28f3bed9d0fd56ce")]
pub struct CancelBid {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CancelBidInstructionAccounts {
    pub bid_state: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub tcomp_program: solana_pubkey::Pubkey,
    pub rent_dest: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelBid {
    type ArrangedAccounts = CancelBidInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let bid_state = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let tcomp_program = next_account(&mut iter)?;
        let rent_dest = next_account(&mut iter)?;

        Some(CancelBidInstructionAccounts {
            bid_state,
            owner,
            system_program,
            tcomp_program,
            rent_dest,
        })
    }
}
