
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xbc23746c00e9edc9")]
pub struct TakeBidLegacy{
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
    pub taker_broker: solana_pubkey::Pubkey,
    pub maker_broker: solana_pubkey::Pubkey,
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

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
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
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(TakeBidLegacyInstructionAccounts {
            tcomp: tcomp.pubkey,
            seller: seller.pubkey,
            bid_state: bid_state.pubkey,
            owner: owner.pubkey,
            taker_broker: taker_broker.pubkey,
            maker_broker: maker_broker.pubkey,
            margin_account: margin_account.pubkey,
            whitelist: whitelist.pubkey,
            nft_seller_acc: nft_seller_acc.pubkey,
            nft_mint: nft_mint.pubkey,
            nft_metadata: nft_metadata.pubkey,
            owner_ata_acc: owner_ata_acc.pubkey,
            nft_edition: nft_edition.pubkey,
            owner_token_record: owner_token_record.pubkey,
            dest_token_record: dest_token_record.pubkey,
            pnft_shared: pnft_shared.pubkey,
            nft_escrow: nft_escrow.pubkey,
            temp_escrow_token_record: temp_escrow_token_record.pubkey,
            auth_rules: auth_rules.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            tcomp_program: tcomp_program.pubkey,
            tensorswap_program: tensorswap_program.pubkey,
            cosigner: cosigner.pubkey,
            mint_proof: mint_proof.pubkey,
            rent_dest: rent_dest.pubkey,
        })
    }
}