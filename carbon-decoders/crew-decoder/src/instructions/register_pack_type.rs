
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x3dfb04702def4bb1")]
pub struct RegisterPackType{
    pub input: RegisterPackTypeInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RegisterPackTypeInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub pack_type: solana_pubkey::Pubkey,
    pub pack_tiers: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub crew_config: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RegisterPackType {
    type ArrangedAccounts = RegisterPackTypeInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            key,
            profile,
            funder,
            pack_type,
            pack_tiers,
            system_program,
            crew_config,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(RegisterPackTypeInstructionAccounts {
            key: key.pubkey,
            profile: profile.pubkey,
            funder: funder.pubkey,
            pack_type: pack_type.pubkey,
            pack_tiers: pack_tiers.pubkey,
            system_program: system_program.pubkey,
            crew_config: crew_config.pubkey,
        })
    }
}