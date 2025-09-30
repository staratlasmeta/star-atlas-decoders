use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x964024c053084939")]
pub struct UpdateCooldownPeriod {
    pub cooldown_period: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateCooldownPeriodInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub registered_stake: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateCooldownPeriod {
    type ArrangedAccounts = UpdateCooldownPeriodInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let authority = next_account(&mut iter)?;
        let registered_stake = next_account(&mut iter)?;

        Some(UpdateCooldownPeriodInstructionAccounts {
            authority,
            registered_stake,
        })
    }
}
