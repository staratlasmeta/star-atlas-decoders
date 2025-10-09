use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbdee65b6ee2f5d1e")]
pub struct AddFeeExemption {
    pub discount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AddFeeExemptionInstructionAccounts {
    pub update_authority_master: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub market_vars_account: solana_pubkey::Pubkey,
    pub fee_exempt_target: solana_pubkey::Pubkey,
    pub fee_exempt_account: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddFeeExemption {
    type ArrangedAccounts = AddFeeExemptionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let update_authority_master = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let market_vars_account = next_account(&mut iter)?;
        let fee_exempt_target = next_account(&mut iter)?;
        let fee_exempt_account = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(AddFeeExemptionInstructionAccounts {
            update_authority_master,
            funder,
            market_vars_account,
            fee_exempt_target,
            fee_exempt_account,
            system_program,
        })
    }
}
