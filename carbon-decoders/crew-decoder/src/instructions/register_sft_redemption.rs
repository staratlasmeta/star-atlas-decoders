
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x36d847fd14ebabf1")]
pub struct RegisterSftRedemption{
    pub input: RegisterSftRedemptionInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RegisterSftRedemptionInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub sft_redemption_data: solana_pubkey::Pubkey,
    pub crew_config: solana_pubkey::Pubkey,
    pub pack_type: solana_pubkey::Pubkey,
    pub sft_mint: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RegisterSftRedemption {
    type ArrangedAccounts = RegisterSftRedemptionInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            key,
            profile,
            funder,
            sft_redemption_data,
            crew_config,
            pack_type,
            sft_mint,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(RegisterSftRedemptionInstructionAccounts {
            key: key.pubkey,
            profile: profile.pubkey,
            funder: funder.pubkey,
            sft_redemption_data: sft_redemption_data.pubkey,
            crew_config: crew_config.pubkey,
            pack_type: pack_type.pubkey,
            sft_mint: sft_mint.pubkey,
            system_program: system_program.pubkey,
        })
    }
}