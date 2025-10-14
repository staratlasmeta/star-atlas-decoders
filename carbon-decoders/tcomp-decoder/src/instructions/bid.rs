
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh, account_utils::next_account};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xc738552692f3259e")]
pub struct Bid{
    pub bid_id: solana_pubkey::Pubkey,
    pub target: Target,
    pub target_id: solana_pubkey::Pubkey,
    pub field: Option<Field>,
    pub field_id: Option<solana_pubkey::Pubkey>,
    pub amount: u64,
    pub quantity: u32,
    pub expire_in_sec: Option<u64>,
    pub currency: Option<solana_pubkey::Pubkey>,
    pub private_taker: Option<solana_pubkey::Pubkey>,
    pub maker_broker: Option<solana_pubkey::Pubkey>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct BidInstructionAccounts {
    pub system_program: solana_pubkey::Pubkey,
    pub tcomp_program: solana_pubkey::Pubkey,
    pub bid_state: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub margin_account: solana_pubkey::Pubkey,
    pub cosigner: solana_pubkey::Pubkey,
    pub rent_payer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Bid {
    type ArrangedAccounts = BidInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let system_program = next_account(&mut iter)?;
        let tcomp_program = next_account(&mut iter)?;
        let bid_state = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let margin_account = next_account(&mut iter)?;
        let cosigner = next_account(&mut iter)?;
        let rent_payer = next_account(&mut iter)?;

        Some(BidInstructionAccounts {
            system_program,
            tcomp_program,
            bid_state,
            owner,
            margin_account,
            cosigner,
            rent_payer,
        })
    }
}