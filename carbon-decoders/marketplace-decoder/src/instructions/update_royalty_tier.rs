use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7b703b7eccb4bfb2")]
pub struct UpdateRoyaltyTier {
    pub stake_amount: u64,
    pub discount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateRoyaltyTierInstructionAccounts {
    pub update_authority_account: solana_pubkey::Pubkey,
    pub market_vars_account: solana_pubkey::Pubkey,
    pub registered_currency: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateRoyaltyTier {
    type ArrangedAccounts = UpdateRoyaltyTierInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let update_authority_account = next_account(&mut iter)?;
        let market_vars_account = next_account(&mut iter)?;
        let registered_currency = next_account(&mut iter)?;

        Some(UpdateRoyaltyTierInstructionAccounts {
            update_authority_account,
            market_vars_account,
            registered_currency,
        })
    }
}
