use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xabb913e6a94fd536")]
pub struct UpdateDomain {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateDomainInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub new_profile: solana_pubkey::Pubkey,
    pub domain: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateDomain {
    type ArrangedAccounts = UpdateDomainInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let new_profile = next_account(&mut iter)?;
        let domain = next_account(&mut iter)?;

        Some(UpdateDomainInstructionAccounts {
            key,
            profile,
            new_profile,
            domain,
        })
    }
}
