use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf2c2cbe1ea350a60")]
pub struct TakeBidFullMeta {
    pub nonce: u64,
    pub index: u32,
    pub root: [u8; 32],
    pub meta_args: TMetadataArgs,
    pub min_amount: u64,
    pub optional_royalty_pct: Option<u16>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TakeBidFullMetaInstructionAccounts {
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
    pub taker_broker: Option<solana_pubkey::Pubkey>,
    pub maker_broker: Option<solana_pubkey::Pubkey>,
    pub margin_account: solana_pubkey::Pubkey,
    pub whitelist: solana_pubkey::Pubkey,
    pub cosigner: solana_pubkey::Pubkey,
    pub rent_dest: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TakeBidFullMeta {
    type ArrangedAccounts = TakeBidFullMetaInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let tcomp = next_account(&mut iter)?;
        let tree_authority = next_account(&mut iter)?;
        let seller = next_account(&mut iter)?;
        let delegate = next_account(&mut iter)?;
        let merkle_tree = next_account(&mut iter)?;
        let log_wrapper = next_account(&mut iter)?;
        let compression_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let bubblegum_program = next_account(&mut iter)?;
        let tcomp_program = next_account(&mut iter)?;
        let tensorswap_program = next_account(&mut iter)?;
        let bid_state = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let taker_broker = next_account(&mut iter);
        let maker_broker = next_account(&mut iter);
        let margin_account = next_account(&mut iter)?;
        let whitelist = next_account(&mut iter)?;
        let cosigner = next_account(&mut iter)?;
        let rent_dest = next_account(&mut iter)?;

        Some(TakeBidFullMetaInstructionAccounts {
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
        })
    }
}
