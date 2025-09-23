use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x724dfc1b34a1029c")]
pub struct FleetStateHandler {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct FleetStateHandlerInstructionAccounts {
    pub fleet: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for FleetStateHandler {
    type ArrangedAccounts = FleetStateHandlerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let fleet = next_account(&mut iter)?;

        Some(FleetStateHandlerInstructionAccounts { fleet })
    }
}
