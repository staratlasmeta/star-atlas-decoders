use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdb9fab7c97f93dbd")]
pub struct RepairIdleFleet {
    pub input: RepairFleetInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RepairIdleFleetInstructionAccounts {
    pub game_and_fleet_and_owner: solana_pubkey::Pubkey,
    pub repaired_fleet: solana_pubkey::Pubkey,
    pub cargo_hold: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub stats_definition: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RepairIdleFleet {
    type ArrangedAccounts = RepairIdleFleetInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_and_fleet_and_owner = next_account(&mut iter)?;
        let repaired_fleet = next_account(&mut iter)?;
        let cargo_hold = next_account(&mut iter)?;
        let cargo_type = next_account(&mut iter)?;
        let stats_definition = next_account(&mut iter)?;
        let token_from = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;
        let cargo_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(RepairIdleFleetInstructionAccounts {
            game_and_fleet_and_owner,
            repaired_fleet,
            cargo_hold,
            cargo_type,
            stats_definition,
            token_from,
            token_mint,
            cargo_program,
            token_program,
        })
    }
}
