
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh, account_utils::next_account};


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
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let rent_recipient = next_account(&mut iter)?;
        let crew_config = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let user_redemption = next_account(&mut iter)?;
        let pack_type = next_account(&mut iter)?;
        let pack_tiers = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let collection_mint = next_account(&mut iter)?;
        let collection_metadata = next_account(&mut iter)?;
        let collection_edition = next_account(&mut iter)?;
        let compression_program = next_account(&mut iter)?;
        let bubblegum_program = next_account(&mut iter)?;
        let log_wrapper = next_account(&mut iter)?;
        let tree_config = next_account(&mut iter)?;
        let merkle_tree = next_account(&mut iter)?;
        let collection_authority_record_pda = next_account(&mut iter)?;
        let server_hash_preimage = next_account(&mut iter)?;

        Some(MintCrewMemberInstructionAccounts {
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
        })
    }
}