

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x3818e702e3130e44")]
pub struct DelistCore{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DelistCoreInstructionAccounts {
    pub asset: solana_pubkey::Pubkey,
    pub collection: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub list_state: solana_pubkey::Pubkey,
    pub mpl_core_program: solana_pubkey::Pubkey,
    pub tcomp_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent_dest: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DelistCore {
    type ArrangedAccounts = DelistCoreInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            asset,
            collection,
            owner,
            list_state,
            mpl_core_program,
            tcomp_program,
            system_program,
            rent_dest,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(DelistCoreInstructionAccounts {
            asset: asset.pubkey,
            collection: collection.pubkey,
            owner: owner.pubkey,
            list_state: list_state.pubkey,
            mpl_core_program: mpl_core_program.pubkey,
            tcomp_program: tcomp_program.pubkey,
            system_program: system_program.pubkey,
            rent_dest: rent_dest.pubkey,
        })
    }
}