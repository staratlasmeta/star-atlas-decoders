use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc96d29db3934b417")]
pub struct TransferCargoWithinFleet {
    pub input: TransferCargoWithinFleetInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TransferCargoWithinFleetInstructionAccounts {
    pub game_accounts_fleet_and_owner: solana_pubkey::Pubkey,
    pub cargo_pod_from: solana_pubkey::Pubkey,
    pub cargo_pod_to: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub token_to: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferCargoWithinFleet {
    type ArrangedAccounts = TransferCargoWithinFleetInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_accounts_fleet_and_owner = next_account(&mut iter)?;
        let cargo_pod_from = next_account(&mut iter)?;
        let cargo_pod_to = next_account(&mut iter)?;
        let cargo_type = next_account(&mut iter)?;
        let cargo_stats_definition = next_account(&mut iter)?;
        let token_from = next_account(&mut iter)?;
        let token_to = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;
        let funds_to = next_account(&mut iter)?;
        let cargo_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(TransferCargoWithinFleetInstructionAccounts {
            game_accounts_fleet_and_owner,
            cargo_pod_from,
            cargo_pod_to,
            cargo_type,
            cargo_stats_definition,
            token_from,
            token_to,
            token_mint,
            funds_to,
            cargo_program,
            token_program,
        })
    }
}
