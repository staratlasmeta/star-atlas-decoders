

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xa9e357ff4c56ff19")]
pub struct BuyCore{
    pub max_amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct BuyCoreInstructionAccounts {
    pub tcomp: solana_pubkey::Pubkey,
    pub list_state: solana_pubkey::Pubkey,
    pub asset: solana_pubkey::Pubkey,
    pub collection: solana_pubkey::Pubkey,
    pub buyer: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub taker_broker: solana_pubkey::Pubkey,
    pub maker_broker: solana_pubkey::Pubkey,
    pub rent_dest: solana_pubkey::Pubkey,
    pub mpl_core_program: solana_pubkey::Pubkey,
    pub tcomp_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BuyCore {
    type ArrangedAccounts = BuyCoreInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            tcomp,
            list_state,
            asset,
            collection,
            buyer,
            payer,
            owner,
            taker_broker,
            maker_broker,
            rent_dest,
            mpl_core_program,
            tcomp_program,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(BuyCoreInstructionAccounts {
            tcomp: tcomp.pubkey,
            list_state: list_state.pubkey,
            asset: asset.pubkey,
            collection: collection.pubkey,
            buyer: buyer.pubkey,
            payer: payer.pubkey,
            owner: owner.pubkey,
            taker_broker: taker_broker.pubkey,
            maker_broker: maker_broker.pubkey,
            rent_dest: rent_dest.pubkey,
            mpl_core_program: mpl_core_program.pubkey,
            tcomp_program: tcomp_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}