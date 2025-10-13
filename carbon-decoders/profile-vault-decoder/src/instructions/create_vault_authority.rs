

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xeaec73fa94e82e07")]
pub struct CreateVaultAuthority{
    pub vault_seed: solana_pubkey::Pubkey,
    pub key_index: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateVaultAuthorityInstructionAccounts {
    pub profile: solana_pubkey::Pubkey,
    pub key: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub vault_authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateVaultAuthority {
    type ArrangedAccounts = CreateVaultAuthorityInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            profile,
            key,
            funder,
            vault_authority,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(CreateVaultAuthorityInstructionAccounts {
            profile: profile.pubkey,
            key: key.pubkey,
            funder: funder.pubkey,
            vault_authority: vault_authority.pubkey,
            system_program: system_program.pubkey,
        })
    }
}