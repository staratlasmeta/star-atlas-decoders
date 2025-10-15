use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8d67117e484b1d1d")]
pub struct CloseVault {
    pub key_index: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseVaultInstructionAccounts {
    pub profile: solana_pubkey::Pubkey,
    pub key: solana_pubkey::Pubkey,
    pub vault: solana_pubkey::Pubkey,
    pub vault_authority: solana_pubkey::Pubkey,
    pub tokens_to: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseVault {
    type ArrangedAccounts = CloseVaultInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let profile = next_account(&mut iter)?;
        let key = next_account(&mut iter)?;
        let vault = next_account(&mut iter)?;
        let vault_authority = next_account(&mut iter)?;
        let tokens_to = next_account(&mut iter)?;
        let funds_to = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(CloseVaultInstructionAccounts {
            profile,
            key,
            vault,
            vault_authority,
            tokens_to,
            funds_to,
            token_program,
        })
    }
}
