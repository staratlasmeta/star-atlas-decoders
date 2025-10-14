

use carbon_core::{CarbonDeserialize, borsh, account_utils::next_account};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x96460d8709cc4b04")]
pub struct CloseExpiredListing{
    pub nonce: u64,
    pub index: u32,
    pub root: [u8; 32],
    pub data_hash: [u8; 32],
    pub creator_hash: [u8; 32],
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseExpiredListingInstructionAccounts {
    pub list_state: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub tcomp_program: solana_pubkey::Pubkey,
    pub tree_authority: solana_pubkey::Pubkey,
    pub merkle_tree: solana_pubkey::Pubkey,
    pub log_wrapper: solana_pubkey::Pubkey,
    pub compression_program: solana_pubkey::Pubkey,
    pub bubblegum_program: solana_pubkey::Pubkey,
    pub rent_dest: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseExpiredListing {
    type ArrangedAccounts = CloseExpiredListingInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let list_state = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let tcomp_program = next_account(&mut iter)?;
        let tree_authority = next_account(&mut iter)?;
        let merkle_tree = next_account(&mut iter)?;
        let log_wrapper = next_account(&mut iter)?;
        let compression_program = next_account(&mut iter)?;
        let bubblegum_program = next_account(&mut iter)?;
        let rent_dest = next_account(&mut iter)?;

        Some(CloseExpiredListingInstructionAccounts {
            list_state,
            owner,
            system_program,
            tcomp_program,
            tree_authority,
            merkle_tree,
            log_wrapper,
            compression_program,
            bubblegum_program,
            rent_dest,
        })
    }
}