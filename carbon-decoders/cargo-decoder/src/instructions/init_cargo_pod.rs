use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4ece42f1e615d7da")]
pub struct InitCargoPod {
    pub pod_seeds: [u8; 32],
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitCargoPodInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub cargo_pod: solana_pubkey::Pubkey,
    pub stats_definition: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitCargoPod {
    type ArrangedAccounts = InitCargoPodInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funder = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let cargo_pod = next_account(&mut iter)?;
        let stats_definition = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(InitCargoPodInstructionAccounts {
            funder,
            authority,
            cargo_pod,
            stats_definition,
            system_program,
        })
    }
}
