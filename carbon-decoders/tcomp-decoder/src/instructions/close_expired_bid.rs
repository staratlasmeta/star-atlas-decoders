

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x53146943f84468be")]
pub struct CloseExpiredBid{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseExpiredBidInstructionAccounts {
    pub bid_state: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub tcomp_program: solana_pubkey::Pubkey,
    pub rent_dest: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseExpiredBid {
    type ArrangedAccounts = CloseExpiredBidInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            bid_state,
            owner,
            system_program,
            tcomp_program,
            rent_dest,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(CloseExpiredBidInstructionAccounts {
            bid_state: bid_state.pubkey,
            owner: owner.pubkey,
            system_program: system_program.pubkey,
            tcomp_program: tcomp_program.pubkey,
            rent_dest: rent_dest.pubkey,
        })
    }
}