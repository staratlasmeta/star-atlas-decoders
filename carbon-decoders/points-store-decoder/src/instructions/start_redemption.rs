use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4dcef5cd74df1c7a")]
pub struct StartRedemption {
    pub input: StartRedemptionInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct StartRedemptionInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub user_profile: solana_pubkey::Pubkey,
    pub profile_faction: solana_pubkey::Pubkey,
    pub point_category: solana_pubkey::Pubkey,
    pub user_points_account: solana_pubkey::Pubkey,
    pub user_redemption_account: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub points_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for StartRedemption {
    type ArrangedAccounts = StartRedemptionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let user_profile = next_account(&mut iter)?;
        let profile_faction = next_account(&mut iter)?;
        let point_category = next_account(&mut iter)?;
        let user_points_account = next_account(&mut iter)?;
        let user_redemption_account = next_account(&mut iter)?;
        let config = next_account(&mut iter)?;
        let points_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(StartRedemptionInstructionAccounts {
            key,
            funder,
            user_profile,
            profile_faction,
            point_category,
            user_points_account,
            user_redemption_account,
            config,
            points_program,
            system_program,
        })
    }
}
