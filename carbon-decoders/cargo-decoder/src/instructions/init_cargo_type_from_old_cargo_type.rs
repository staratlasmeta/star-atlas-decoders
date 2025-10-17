use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb278baef0dcadc9b")]
pub struct InitCargoTypeFromOldCargoType {
    pub input: InitCargoTypeFromOldCargoTypeInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitCargoTypeFromOldCargoTypeInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub stats_definition: solana_pubkey::Pubkey,
    pub old_cargo_type: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitCargoTypeFromOldCargoType {
    type ArrangedAccounts = InitCargoTypeFromOldCargoTypeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let stats_definition = next_account(&mut iter)?;
        let old_cargo_type = next_account(&mut iter)?;
        let cargo_type = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(InitCargoTypeFromOldCargoTypeInstructionAccounts {
            key,
            profile,
            funder,
            stats_definition,
            old_cargo_type,
            cargo_type,
            system_program,
        })
    }
}
