

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x4188feff3b82eaae")]
pub struct BuySpl{
    pub nonce: u64,
    pub index: u32,
    pub root: [u8; 32],
    pub meta_hash: [u8; 32],
    pub creator_shares: Vec<u8>,
    pub creator_verified: Vec<bool>,
    pub seller_fee_basis_points: u16,
    pub max_amount: u64,
    pub optional_royalty_pct: Option<u16>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct BuySplInstructionAccounts {
    pub tcomp: solana_pubkey::Pubkey,
    pub tcomp_ata: solana_pubkey::Pubkey,
    pub tree_authority: solana_pubkey::Pubkey,
    pub merkle_tree: solana_pubkey::Pubkey,
    pub log_wrapper: solana_pubkey::Pubkey,
    pub compression_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub bubblegum_program: solana_pubkey::Pubkey,
    pub tcomp_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub list_state: solana_pubkey::Pubkey,
    pub buyer: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub payer_source: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub owner_dest: solana_pubkey::Pubkey,
    pub currency: solana_pubkey::Pubkey,
    pub taker_broker: solana_pubkey::Pubkey,
    pub taker_broker_ata: solana_pubkey::Pubkey,
    pub maker_broker: solana_pubkey::Pubkey,
    pub maker_broker_ata: solana_pubkey::Pubkey,
    pub rent_dest: solana_pubkey::Pubkey,
    pub rent_payer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BuySpl {
    type ArrangedAccounts = BuySplInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            tcomp,
            tcomp_ata,
            tree_authority,
            merkle_tree,
            log_wrapper,
            compression_program,
            system_program,
            bubblegum_program,
            tcomp_program,
            token_program,
            associated_token_program,
            list_state,
            buyer,
            payer,
            payer_source,
            owner,
            owner_dest,
            currency,
            taker_broker,
            taker_broker_ata,
            maker_broker,
            maker_broker_ata,
            rent_dest,
            rent_payer,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(BuySplInstructionAccounts {
            tcomp: tcomp.pubkey,
            tcomp_ata: tcomp_ata.pubkey,
            tree_authority: tree_authority.pubkey,
            merkle_tree: merkle_tree.pubkey,
            log_wrapper: log_wrapper.pubkey,
            compression_program: compression_program.pubkey,
            system_program: system_program.pubkey,
            bubblegum_program: bubblegum_program.pubkey,
            tcomp_program: tcomp_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            list_state: list_state.pubkey,
            buyer: buyer.pubkey,
            payer: payer.pubkey,
            payer_source: payer_source.pubkey,
            owner: owner.pubkey,
            owner_dest: owner_dest.pubkey,
            currency: currency.pubkey,
            taker_broker: taker_broker.pubkey,
            taker_broker_ata: taker_broker_ata.pubkey,
            maker_broker: maker_broker.pubkey,
            maker_broker_ata: maker_broker_ata.pubkey,
            rent_dest: rent_dest.pubkey,
            rent_payer: rent_payer.pubkey,
        })
    }
}