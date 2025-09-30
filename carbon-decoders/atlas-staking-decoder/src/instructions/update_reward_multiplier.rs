use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfcd293f6c4f24cb0")]
pub struct UpdateRewardMultiplier {
    pub reward_multiplier: u64,
    pub new_staking_period: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateRewardMultiplierInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub registered_stake: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateRewardMultiplier {
    type ArrangedAccounts = UpdateRewardMultiplierInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let authority = next_account(&mut iter)?;
        let registered_stake = next_account(&mut iter)?;

        Some(UpdateRewardMultiplierInstructionAccounts {
            authority,
            registered_stake,
        })
    }
}
