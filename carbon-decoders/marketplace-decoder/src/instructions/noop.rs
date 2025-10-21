use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x09b20d738123ed66")]
pub struct Noop {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct NoopInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for Noop {
    type ArrangedAccounts = NoopInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        Some(NoopInstructionAccounts {})
    }
}
