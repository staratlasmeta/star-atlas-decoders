use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x57db22f905874174")]
pub struct DrainVault {
    pub key_index: u16,
    pub amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DrainVaultInstructionAccounts {
    pub profile: solana_pubkey::Pubkey,
    pub key: solana_pubkey::Pubkey,
    pub vault: solana_pubkey::Pubkey,
    pub vault_authority: solana_pubkey::Pubkey,
    pub tokens_to: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DrainVault {
    type ArrangedAccounts = DrainVaultInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let profile = next_account(&mut iter)?;
        let key = next_account(&mut iter)?;
        let vault = next_account(&mut iter)?;
        let vault_authority = next_account(&mut iter)?;
        let tokens_to = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(DrainVaultInstructionAccounts {
            profile,
            key,
            vault,
            vault_authority,
            tokens_to,
            token_program,
        })
    }
}
