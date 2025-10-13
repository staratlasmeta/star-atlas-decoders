

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xad4ca77d76470199")]
pub struct ListCore{
    pub amount: u64,
    pub expire_in_sec: Option<u64>,
    pub currency: Option<solana_pubkey::Pubkey>,
    pub private_taker: Option<solana_pubkey::Pubkey>,
    pub maker_broker: Option<solana_pubkey::Pubkey>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ListCoreInstructionAccounts {
    pub asset: solana_pubkey::Pubkey,
    pub collection: solana_pubkey::Pubkey,
    pub list_state: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub mpl_core_program: solana_pubkey::Pubkey,
    pub tcomp_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ListCore {
    type ArrangedAccounts = ListCoreInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            asset,
            collection,
            list_state,
            owner,
            mpl_core_program,
            tcomp_program,
            system_program,
            payer,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(ListCoreInstructionAccounts {
            asset: asset.pubkey,
            collection: collection.pubkey,
            list_state: list_state.pubkey,
            owner: owner.pubkey,
            mpl_core_program: mpl_core_program.pubkey,
            tcomp_program: tcomp_program.pubkey,
            system_program: system_program.pubkey,
            payer: payer.pubkey,
        })
    }
}