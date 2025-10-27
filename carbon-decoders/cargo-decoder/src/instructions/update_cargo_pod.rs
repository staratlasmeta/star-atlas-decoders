use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x93c2fdeae85d2392")]
pub struct UpdateCargoPod {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateCargoPodInstructionAccounts {
    pub cargo_pod: solana_pubkey::Pubkey,
    pub stats_definition: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateCargoPod {
    type ArrangedAccounts = UpdateCargoPodInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let cargo_pod = next_account(&mut iter)?;
        let stats_definition = next_account(&mut iter)?;

        Some(UpdateCargoPodInstructionAccounts {
            cargo_pod,
            stats_definition,
        })
    }
}
