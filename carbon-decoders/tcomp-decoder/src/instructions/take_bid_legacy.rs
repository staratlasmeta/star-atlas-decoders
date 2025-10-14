use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbc23746c00e9edc9")]
pub struct TakeBidLegacy {
    pub min_amount: u64,
    pub optional_royalty_pct: Option<u16>,
    pub rules_acc_present: bool,
    pub authorization_data: Option<AuthorizationDataLocal>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TakeBidLegacyInstructionAccounts {
    pub tcomp: solana_pubkey::Pubkey,
    pub seller: solana_pubkey::Pubkey,
    pub bid_state: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub taker_broker: Option<solana_pubkey::Pubkey>,
    pub maker_broker: Option<solana_pubkey::Pubkey>,
    pub margin_account: solana_pubkey::Pubkey,
    pub whitelist: solana_pubkey::Pubkey,
    pub nft_seller_acc: solana_pubkey::Pubkey,
    pub nft_mint: solana_pubkey::Pubkey,
    pub nft_metadata: solana_pubkey::Pubkey,
    pub owner_ata_acc: solana_pubkey::Pubkey,
    pub nft_edition: solana_pubkey::Pubkey,
    pub owner_token_record: solana_pubkey::Pubkey,
    pub dest_token_record: solana_pubkey::Pubkey,
    pub pnft_shared: solana_pubkey::Pubkey,
    pub nft_escrow: solana_pubkey::Pubkey,
    pub temp_escrow_token_record: solana_pubkey::Pubkey,
    pub auth_rules: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub tcomp_program: solana_pubkey::Pubkey,
    pub tensorswap_program: solana_pubkey::Pubkey,
    pub cosigner: solana_pubkey::Pubkey,
    pub mint_proof: solana_pubkey::Pubkey,
    pub rent_dest: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TakeBidLegacy {
    type ArrangedAccounts = TakeBidLegacyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let tcomp = next_account(&mut iter)?;
        let seller = next_account(&mut iter)?;
        let bid_state = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let taker_broker = next_account(&mut iter);
        let maker_broker = next_account(&mut iter);
        let margin_account = next_account(&mut iter)?;
        let whitelist = next_account(&mut iter)?;
        let nft_seller_acc = next_account(&mut iter)?;
        let nft_mint = next_account(&mut iter)?;
        let nft_metadata = next_account(&mut iter)?;
        let owner_ata_acc = next_account(&mut iter)?;
        let nft_edition = next_account(&mut iter)?;
        let owner_token_record = next_account(&mut iter)?;
        let dest_token_record = next_account(&mut iter)?;
        let pnft_shared = next_account(&mut iter)?;
        let nft_escrow = next_account(&mut iter)?;
        let temp_escrow_token_record = next_account(&mut iter)?;
        let auth_rules = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let tcomp_program = next_account(&mut iter)?;
        let tensorswap_program = next_account(&mut iter)?;
        let cosigner = next_account(&mut iter)?;
        let mint_proof = next_account(&mut iter)?;
        let rent_dest = next_account(&mut iter)?;

        Some(TakeBidLegacyInstructionAccounts {
            tcomp,
            seller,
            bid_state,
            owner,
            taker_broker,
            maker_broker,
            margin_account,
            whitelist,
            nft_seller_acc,
            nft_mint,
            nft_metadata,
            owner_ata_acc,
            nft_edition,
            owner_token_record,
            dest_token_record,
            pnft_shared,
            nft_escrow,
            temp_escrow_token_record,
            auth_rules,
            token_program,
            associated_token_program,
            system_program,
            tcomp_program,
            tensorswap_program,
            cosigner,
            mint_proof,
            rent_dest,
        })
    }
}
