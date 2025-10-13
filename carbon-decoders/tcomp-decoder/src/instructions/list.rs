

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x36aec14311298426")]
pub struct List{
    pub nonce: u64,
    pub index: u32,
    pub root: [u8; 32],
    pub data_hash: [u8; 32],
    pub creator_hash: [u8; 32],
    pub amount: u64,
    pub expire_in_sec: Option<u64>,
    pub currency: Option<solana_pubkey::Pubkey>,
    pub private_taker: Option<solana_pubkey::Pubkey>,
    pub maker_broker: Option<solana_pubkey::Pubkey>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ListInstructionAccounts {
    pub tree_authority: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub delegate: solana_pubkey::Pubkey,
    pub merkle_tree: solana_pubkey::Pubkey,
    pub log_wrapper: solana_pubkey::Pubkey,
    pub compression_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub bubblegum_program: solana_pubkey::Pubkey,
    pub tcomp_program: solana_pubkey::Pubkey,
    pub list_state: solana_pubkey::Pubkey,
    pub rent_payer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for List {
    type ArrangedAccounts = ListInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            tree_authority,
            owner,
            delegate,
            merkle_tree,
            log_wrapper,
            compression_program,
            system_program,
            bubblegum_program,
            tcomp_program,
            list_state,
            rent_payer,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(ListInstructionAccounts {
            tree_authority: tree_authority.pubkey,
            owner: owner.pubkey,
            delegate: delegate.pubkey,
            merkle_tree: merkle_tree.pubkey,
            log_wrapper: log_wrapper.pubkey,
            compression_program: compression_program.pubkey,
            system_program: system_program.pubkey,
            bubblegum_program: bubblegum_program.pubkey,
            tcomp_program: tcomp_program.pubkey,
            list_state: list_state.pubkey,
            rent_payer: rent_payer.pubkey,
        })
    }
}