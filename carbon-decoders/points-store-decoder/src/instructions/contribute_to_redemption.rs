use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7b6f0fb867fc1120")]
pub struct ContributeToRedemption {
    pub input: ContributeToRedemptionInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ContributeToRedemptionInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub profile_faction: solana_pubkey::Pubkey,
    pub point_category: solana_pubkey::Pubkey,
    pub user_points_account: solana_pubkey::Pubkey,
    pub user_redemption_account: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub points_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ContributeToRedemption {
    type ArrangedAccounts = ContributeToRedemptionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let profile_faction = next_account(&mut iter)?;
        let point_category = next_account(&mut iter)?;
        let user_points_account = next_account(&mut iter)?;
        let user_redemption_account = next_account(&mut iter)?;
        let config = next_account(&mut iter)?;
        let points_program = next_account(&mut iter)?;

        Some(ContributeToRedemptionInstructionAccounts {
            key,
            profile,
            profile_faction,
            point_category,
            user_points_account,
            user_redemption_account,
            config,
            points_program,
        })
    }
}
