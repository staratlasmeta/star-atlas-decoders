use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xda4710ee0c269587")]
pub struct CloseCargoPod {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseCargoPodInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub cargo_pod: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseCargoPod {
    type ArrangedAccounts = CloseCargoPodInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funder = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let cargo_pod = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CloseCargoPodInstructionAccounts {
            funder,
            authority,
            cargo_pod,
            system_program,
        })
    }
}
