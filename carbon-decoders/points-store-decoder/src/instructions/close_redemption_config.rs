use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb7cfc6599a64ef18")]
pub struct CloseRedemptionConfig {
    pub key_index: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseRedemptionConfigInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub config_signer: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub tokens_to: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseRedemptionConfig {
    type ArrangedAccounts = CloseRedemptionConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let config = next_account(&mut iter)?;
        let config_signer = next_account(&mut iter)?;
        let bank = next_account(&mut iter)?;
        let funds_to = next_account(&mut iter)?;
        let tokens_to = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CloseRedemptionConfigInstructionAccounts {
            key,
            profile,
            config,
            config_signer,
            bank,
            funds_to,
            tokens_to,
            token_program,
            system_program,
        })
    }
}
