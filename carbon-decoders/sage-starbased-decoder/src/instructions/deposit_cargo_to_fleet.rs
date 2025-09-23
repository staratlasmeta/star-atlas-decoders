use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x37eb697b00fd40ed")]
pub struct DepositCargoToFleet {
    pub input: DepositCargoToFleetInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DepositCargoToFleetInstructionAccounts {
    pub game_accounts_fleet_and_owner: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub cargo_pod_from: solana_pubkey::Pubkey,
    pub cargo_pod_to: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub token_to: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositCargoToFleet {
    type ArrangedAccounts = DepositCargoToFleetInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_accounts_fleet_and_owner = next_account(&mut iter)?;
        let funds_to = next_account(&mut iter)?;
        let starbase_and_starbase_player = next_account(&mut iter)?;
        let cargo_pod_from = next_account(&mut iter)?;
        let cargo_pod_to = next_account(&mut iter)?;
        let cargo_type = next_account(&mut iter)?;
        let cargo_stats_definition = next_account(&mut iter)?;
        let token_from = next_account(&mut iter)?;
        let token_to = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;
        let cargo_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(DepositCargoToFleetInstructionAccounts {
            game_accounts_fleet_and_owner,
            funds_to,
            starbase_and_starbase_player,
            cargo_pod_from,
            cargo_pod_to,
            cargo_type,
            cargo_stats_definition,
            token_from,
            token_to,
            token_mint,
            cargo_program,
            token_program,
        })
    }
}
