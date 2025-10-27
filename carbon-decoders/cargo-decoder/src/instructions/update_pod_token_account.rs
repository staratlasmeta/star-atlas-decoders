use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb1750f5f60ae3618")]
pub struct UpdatePodTokenAccount {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdatePodTokenAccountInstructionAccounts {
    pub stats_definition: solana_pubkey::Pubkey,
    pub cargo_pod: solana_pubkey::Pubkey,
    pub old_cargo_type: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub cargo_token_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePodTokenAccount {
    type ArrangedAccounts = UpdatePodTokenAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let stats_definition = next_account(&mut iter)?;
        let cargo_pod = next_account(&mut iter)?;
        let old_cargo_type = next_account(&mut iter)?;
        let cargo_type = next_account(&mut iter)?;
        let cargo_token_account = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(UpdatePodTokenAccountInstructionAccounts {
            stats_definition,
            cargo_pod,
            old_cargo_type,
            cargo_type,
            cargo_token_account,
            token_program,
        })
    }
}
