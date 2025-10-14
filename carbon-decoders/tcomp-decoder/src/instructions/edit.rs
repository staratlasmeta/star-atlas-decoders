

use carbon_core::{CarbonDeserialize, borsh, account_utils::next_account};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0fb72156571c9791")]
pub struct Edit{
    pub amount: u64,
    pub expire_in_sec: Option<u64>,
    pub currency: Option<solana_pubkey::Pubkey>,
    pub private_taker: Option<solana_pubkey::Pubkey>,
    pub maker_broker: Option<solana_pubkey::Pubkey>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct EditInstructionAccounts {
    pub list_state: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub tcomp_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Edit {
    type ArrangedAccounts = EditInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let list_state = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let tcomp_program = next_account(&mut iter)?;

        Some(EditInstructionAccounts {
            list_state,
            owner,
            tcomp_program,
        })
    }
}