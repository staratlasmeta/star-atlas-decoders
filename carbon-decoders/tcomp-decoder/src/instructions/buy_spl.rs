

use carbon_core::{CarbonDeserialize, borsh, account_utils::next_account};


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
    pub taker_broker: Option<solana_pubkey::Pubkey>,
    pub taker_broker_ata: Option<solana_pubkey::Pubkey>,
    pub maker_broker: Option<solana_pubkey::Pubkey>,
    pub maker_broker_ata: Option<solana_pubkey::Pubkey>,
    pub rent_dest: solana_pubkey::Pubkey,
    pub rent_payer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BuySpl {
    type ArrangedAccounts = BuySplInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let tcomp = next_account(&mut iter)?;
        let tcomp_ata = next_account(&mut iter)?;
        let tree_authority = next_account(&mut iter)?;
        let merkle_tree = next_account(&mut iter)?;
        let log_wrapper = next_account(&mut iter)?;
        let compression_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let bubblegum_program = next_account(&mut iter)?;
        let tcomp_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let list_state = next_account(&mut iter)?;
        let buyer = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let payer_source = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let owner_dest = next_account(&mut iter)?;
        let currency = next_account(&mut iter)?;
        let taker_broker = next_account(&mut iter);
        let taker_broker_ata = next_account(&mut iter);
        let maker_broker = next_account(&mut iter);
        let maker_broker_ata = next_account(&mut iter);
        let rent_dest = next_account(&mut iter)?;
        let rent_payer = next_account(&mut iter)?;

        Some(BuySplInstructionAccounts {
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
        })
    }
}