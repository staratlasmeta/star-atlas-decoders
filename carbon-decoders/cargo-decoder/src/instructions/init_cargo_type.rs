use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x55c62d1029a3d727")]
pub struct InitCargoType {
    pub input: InitCargoTypeInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitCargoTypeInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub stats_definition: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitCargoType {
    type ArrangedAccounts = InitCargoTypeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let stats_definition = next_account(&mut iter)?;
        let cargo_type = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(InitCargoTypeInstructionAccounts {
            key,
            profile,
            funder,
            mint,
            stats_definition,
            cargo_type,
            system_program,
        })
    }
}
