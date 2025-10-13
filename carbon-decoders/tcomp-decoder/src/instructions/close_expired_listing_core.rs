

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x59ab4e504abc3f3a")]
pub struct CloseExpiredListingCore{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseExpiredListingCoreInstructionAccounts {
    pub list_state: solana_pubkey::Pubkey,
    pub asset: solana_pubkey::Pubkey,
    pub collection: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub mpl_core_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub tcomp_program: solana_pubkey::Pubkey,
    pub rent_dest: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseExpiredListingCore {
    type ArrangedAccounts = CloseExpiredListingCoreInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            list_state,
            asset,
            collection,
            owner,
            mpl_core_program,
            system_program,
            tcomp_program,
            rent_dest,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(CloseExpiredListingCoreInstructionAccounts {
            list_state: list_state.pubkey,
            asset: asset.pubkey,
            collection: collection.pubkey,
            owner: owner.pubkey,
            mpl_core_program: mpl_core_program.pubkey,
            system_program: system_program.pubkey,
            tcomp_program: tcomp_program.pubkey,
            rent_dest: rent_dest.pubkey,
        })
    }
}