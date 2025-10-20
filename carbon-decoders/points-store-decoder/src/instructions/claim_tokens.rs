use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6cd8d2e700d42a40")]
pub struct ClaimTokens {
    pub input: ClaimTokensInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimTokensInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub user_profile: solana_pubkey::Pubkey,
    pub user_redemption_account: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub config_signer: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
    pub tokens_to: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimTokens {
    type ArrangedAccounts = ClaimTokensInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let funds_to = next_account(&mut iter)?;
        let user_profile = next_account(&mut iter)?;
        let user_redemption_account = next_account(&mut iter)?;
        let config = next_account(&mut iter)?;
        let config_signer = next_account(&mut iter)?;
        let bank = next_account(&mut iter)?;
        let tokens_to = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(ClaimTokensInstructionAccounts {
            key,
            funds_to,
            user_profile,
            user_redemption_account,
            config,
            config_signer,
            bank,
            tokens_to,
            token_program,
        })
    }
}
