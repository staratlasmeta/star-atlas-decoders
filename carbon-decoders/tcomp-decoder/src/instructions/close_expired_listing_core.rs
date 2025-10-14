

use carbon_core::{CarbonDeserialize, borsh, account_utils::next_account};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x59ab4e504abc3f3a")]
pub struct CloseExpiredListingCore{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseExpiredListingCoreInstructionAccounts {
    pub list_state: solana_pubkey::Pubkey,
    pub asset: solana_pubkey::Pubkey,
    pub collection: Option<solana_pubkey::Pubkey>,
    pub owner: solana_pubkey::Pubkey,
    pub mpl_core_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub tcomp_program: solana_pubkey::Pubkey,
    pub rent_dest: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseExpiredListingCore {
    type ArrangedAccounts = CloseExpiredListingCoreInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let list_state = next_account(&mut iter)?;
        let asset = next_account(&mut iter)?;
        let collection = next_account(&mut iter);
        let owner = next_account(&mut iter)?;
        let mpl_core_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let tcomp_program = next_account(&mut iter)?;
        let rent_dest = next_account(&mut iter)?;

        Some(CloseExpiredListingCoreInstructionAccounts {
            list_state,
            asset,
            collection,
            owner,
            mpl_core_program,
            system_program,
            tcomp_program,
            rent_dest,
        })
    }
}