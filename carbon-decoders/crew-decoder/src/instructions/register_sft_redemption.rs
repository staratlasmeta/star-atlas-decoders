use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x36d847fd14ebabf1")]
pub struct RegisterSftRedemption {
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

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let sft_redemption_data = next_account(&mut iter)?;
        let crew_config = next_account(&mut iter)?;
        let pack_type = next_account(&mut iter)?;
        let sft_mint = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RegisterSftRedemptionInstructionAccounts {
            key,
            profile,
            funder,
            sft_redemption_data,
            crew_config,
            pack_type,
            sft_mint,
            system_program,
        })
    }
}
