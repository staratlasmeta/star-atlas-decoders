use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9e3b188b1d8d3f0f")]
pub struct RemoveFeeExemption {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RemoveFeeExemptionInstructionAccounts {
    pub update_authority_master: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub market_vars_account: solana_pubkey::Pubkey,
    pub fee_exempt_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveFeeExemption {
    type ArrangedAccounts = RemoveFeeExemptionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let update_authority_master = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let market_vars_account = next_account(&mut iter)?;
        let fee_exempt_account = next_account(&mut iter)?;

        Some(RemoveFeeExemptionInstructionAccounts {
            update_authority_master,
            funder,
            market_vars_account,
            fee_exempt_account,
        })
    }
}
