use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xeaec73fa94e82e07")]
pub struct CreateVaultAuthority {
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

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let profile = next_account(&mut iter)?;
        let key = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let vault_authority = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CreateVaultAuthorityInstructionAccounts {
            profile,
            key,
            funder,
            vault_authority,
            system_program,
        })
    }
}
