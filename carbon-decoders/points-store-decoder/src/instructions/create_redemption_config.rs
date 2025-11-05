use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x042b9ec1b87a527c")]
pub struct CreateRedemptionConfig {
    pub input: CreateRedemptionConfigInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateRedemptionConfigInstructionAccounts {
    pub profile: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub config_signer: solana_pubkey::Pubkey,
    pub point_category: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateRedemptionConfig {
    type ArrangedAccounts = CreateRedemptionConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let profile = next_account(&mut iter)?;
        let config = next_account(&mut iter)?;
        let config_signer = next_account(&mut iter)?;
        let point_category = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let bank = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CreateRedemptionConfigInstructionAccounts {
            profile,
            config,
            config_signer,
            point_category,
            funder,
            bank,
            system_program,
        })
    }
}
