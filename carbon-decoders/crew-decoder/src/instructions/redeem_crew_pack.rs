
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x84c6f3296e99c7e8")]
pub struct RedeemCrewPack{
    pub input: RedeemCrewPacksInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RedeemCrewPackInstructionAccounts {
    pub pack_sft_authority: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub profile_key: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub user_redemption: solana_pubkey::Pubkey,
    pub sft_redemption: solana_pubkey::Pubkey,
    pub pack_type: solana_pubkey::Pubkey,
    pub pack_tiers: solana_pubkey::Pubkey,
    pub crew_config: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub instructions_sysvar: solana_pubkey::Pubkey,
    pub recent_slothashes: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RedeemCrewPack {
    type ArrangedAccounts = RedeemCrewPackInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            pack_sft_authority,
            profile,
            profile_key,
            funder,
            owner,
            user_redemption,
            sft_redemption,
            pack_type,
            pack_tiers,
            crew_config,
            token_from,
            token_mint,
            token_program,
            instructions_sysvar,
            recent_slothashes,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(RedeemCrewPackInstructionAccounts {
            pack_sft_authority: pack_sft_authority.pubkey,
            profile: profile.pubkey,
            profile_key: profile_key.pubkey,
            funder: funder.pubkey,
            owner: owner.pubkey,
            user_redemption: user_redemption.pubkey,
            sft_redemption: sft_redemption.pubkey,
            pack_type: pack_type.pubkey,
            pack_tiers: pack_tiers.pubkey,
            crew_config: crew_config.pubkey,
            token_from: token_from.pubkey,
            token_mint: token_mint.pubkey,
            token_program: token_program.pubkey,
            instructions_sysvar: instructions_sysvar.pubkey,
            recent_slothashes: recent_slothashes.pubkey,
            system_program: system_program.pubkey,
        })
    }
}