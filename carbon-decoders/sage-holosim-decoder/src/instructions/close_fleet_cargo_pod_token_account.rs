use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4e7cf7a394fd8e44")]
pub struct CloseFleetCargoPodTokenAccount {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseFleetCargoPodTokenAccountInstructionAccounts {
    pub game_accounts_fleet_and_owner: solana_pubkey::Pubkey,
    pub cargo_pod: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub token: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseFleetCargoPodTokenAccount {
    type ArrangedAccounts = CloseFleetCargoPodTokenAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_accounts_fleet_and_owner = next_account(&mut iter)?;
        let cargo_pod = next_account(&mut iter)?;
        let cargo_type = next_account(&mut iter)?;
        let cargo_stats_definition = next_account(&mut iter)?;
        let token = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;
        let funds_to = next_account(&mut iter)?;
        let cargo_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(CloseFleetCargoPodTokenAccountInstructionAccounts {
            game_accounts_fleet_and_owner,
            cargo_pod,
            cargo_type,
            cargo_stats_definition,
            token,
            token_mint,
            funds_to,
            cargo_program,
            token_program,
        })
    }
}
