use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7cb765f73feadba5")]
pub struct InitializeDomain {
    pub namespace: [u8; 32],
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeDomainInstructionAccounts {
    pub signer: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub domain: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeDomain {
    type ArrangedAccounts = InitializeDomainInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let signer = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let domain = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(InitializeDomainInstructionAccounts {
            signer,
            profile,
            funder,
            domain,
            system_program,
        })
    }
}
