
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x4149c4ae99bdad32")]
pub struct MintCrewMember{
    pub input: MintCrewMemberInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MintCrewMemberInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub rent_recipient: solana_pubkey::Pubkey,
    pub crew_config: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub user_redemption: solana_pubkey::Pubkey,
    pub pack_type: solana_pubkey::Pubkey,
    pub pack_tiers: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub collection_mint: solana_pubkey::Pubkey,
    pub collection_metadata: solana_pubkey::Pubkey,
    pub collection_edition: solana_pubkey::Pubkey,
    pub compression_program: solana_pubkey::Pubkey,
    pub bubblegum_program: solana_pubkey::Pubkey,
    pub log_wrapper: solana_pubkey::Pubkey,
    pub tree_config: solana_pubkey::Pubkey,
    pub merkle_tree: solana_pubkey::Pubkey,
    pub collection_authority_record_pda: solana_pubkey::Pubkey,
    pub server_hash_preimage: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MintCrewMember {
    type ArrangedAccounts = MintCrewMemberInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            key,
            profile,
            rent_recipient,
            crew_config,
            owner,
            user_redemption,
            pack_type,
            pack_tiers,
            system_program,
            collection_mint,
            collection_metadata,
            collection_edition,
            compression_program,
            bubblegum_program,
            log_wrapper,
            tree_config,
            merkle_tree,
            collection_authority_record_pda,
            server_hash_preimage,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(MintCrewMemberInstructionAccounts {
            key: key.pubkey,
            profile: profile.pubkey,
            rent_recipient: rent_recipient.pubkey,
            crew_config: crew_config.pubkey,
            owner: owner.pubkey,
            user_redemption: user_redemption.pubkey,
            pack_type: pack_type.pubkey,
            pack_tiers: pack_tiers.pubkey,
            system_program: system_program.pubkey,
            collection_mint: collection_mint.pubkey,
            collection_metadata: collection_metadata.pubkey,
            collection_edition: collection_edition.pubkey,
            compression_program: compression_program.pubkey,
            bubblegum_program: bubblegum_program.pubkey,
            log_wrapper: log_wrapper.pubkey,
            tree_config: tree_config.pubkey,
            merkle_tree: merkle_tree.pubkey,
            collection_authority_record_pda: collection_authority_record_pda.pubkey,
            server_hash_preimage: server_hash_preimage.pubkey,
        })
    }
}