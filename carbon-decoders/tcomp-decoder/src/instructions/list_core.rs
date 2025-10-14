

use carbon_core::{CarbonDeserialize, borsh, account_utils::next_account};


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
    pub collection: Option<solana_pubkey::Pubkey>,
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
        let mut iter = accounts.iter();
        let asset = next_account(&mut iter)?;
        let collection = next_account(&mut iter);
        let list_state = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let mpl_core_program = next_account(&mut iter)?;
        let tcomp_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;

        Some(ListCoreInstructionAccounts {
            asset,
            collection,
            list_state,
            owner,
            mpl_core_program,
            tcomp_program,
            system_program,
            payer,
        })
    }
}