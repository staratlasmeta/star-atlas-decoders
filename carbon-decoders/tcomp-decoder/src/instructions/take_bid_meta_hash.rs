

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x55e3ca462dd70ac1")]
pub struct TakeBidMetaHash{
    pub nonce: u64,
    pub index: u32,
    pub root: [u8; 32],
    pub meta_hash: [u8; 32],
    pub creator_shares: Vec<u8>,
    pub creator_verified: Vec<bool>,
    pub seller_fee_basis_points: u16,
    pub min_amount: u64,
    pub optional_royalty_pct: Option<u16>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TakeBidMetaHashInstructionAccounts {
    pub tcomp: solana_pubkey::Pubkey,
    pub tree_authority: solana_pubkey::Pubkey,
    pub seller: solana_pubkey::Pubkey,
    pub delegate: solana_pubkey::Pubkey,
    pub merkle_tree: solana_pubkey::Pubkey,
    pub log_wrapper: solana_pubkey::Pubkey,
    pub compression_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub bubblegum_program: solana_pubkey::Pubkey,
    pub tcomp_program: solana_pubkey::Pubkey,
    pub tensorswap_program: solana_pubkey::Pubkey,
    pub bid_state: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub taker_broker: solana_pubkey::Pubkey,
    pub maker_broker: solana_pubkey::Pubkey,
    pub margin_account: solana_pubkey::Pubkey,
    pub whitelist: solana_pubkey::Pubkey,
    pub cosigner: solana_pubkey::Pubkey,
    pub rent_dest: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TakeBidMetaHash {
    type ArrangedAccounts = TakeBidMetaHashInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            tcomp,
            tree_authority,
            seller,
            delegate,
            merkle_tree,
            log_wrapper,
            compression_program,
            system_program,
            bubblegum_program,
            tcomp_program,
            tensorswap_program,
            bid_state,
            owner,
            taker_broker,
            maker_broker,
            margin_account,
            whitelist,
            cosigner,
            rent_dest,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(TakeBidMetaHashInstructionAccounts {
            tcomp: tcomp.pubkey,
            tree_authority: tree_authority.pubkey,
            seller: seller.pubkey,
            delegate: delegate.pubkey,
            merkle_tree: merkle_tree.pubkey,
            log_wrapper: log_wrapper.pubkey,
            compression_program: compression_program.pubkey,
            system_program: system_program.pubkey,
            bubblegum_program: bubblegum_program.pubkey,
            tcomp_program: tcomp_program.pubkey,
            tensorswap_program: tensorswap_program.pubkey,
            bid_state: bid_state.pubkey,
            owner: owner.pubkey,
            taker_broker: taker_broker.pubkey,
            maker_broker: maker_broker.pubkey,
            margin_account: margin_account.pubkey,
            whitelist: whitelist.pubkey,
            cosigner: cosigner.pubkey,
            rent_dest: rent_dest.pubkey,
        })
    }
}