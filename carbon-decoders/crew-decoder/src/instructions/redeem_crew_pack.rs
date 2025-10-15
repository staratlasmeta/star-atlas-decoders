use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x84c6f3296e99c7e8")]
pub struct RedeemCrewPack {
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

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let pack_sft_authority = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let profile_key = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let user_redemption = next_account(&mut iter)?;
        let sft_redemption = next_account(&mut iter)?;
        let pack_type = next_account(&mut iter)?;
        let pack_tiers = next_account(&mut iter)?;
        let crew_config = next_account(&mut iter)?;
        let token_from = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let instructions_sysvar = next_account(&mut iter)?;
        let recent_slothashes = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RedeemCrewPackInstructionAccounts {
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
        })
    }
}
