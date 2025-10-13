

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x57db22f905874174")]
pub struct DrainVault{
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

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            profile,
            key,
            vault,
            vault_authority,
            tokens_to,
            token_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(DrainVaultInstructionAccounts {
            profile: profile.pubkey,
            key: key.pubkey,
            vault: vault.pubkey,
            vault_authority: vault_authority.pubkey,
            tokens_to: tokens_to.pubkey,
            token_program: token_program.pubkey,
        })
    }
}