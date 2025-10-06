use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb54d2da3671bd351")]
pub struct StopMiningAsteroid {
    pub input: StopMiningAsteroidInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct StopMiningAsteroidInstructionAccounts {
    pub game_accounts_fleet_and_owner: solana_pubkey::Pubkey,
    pub mine_item: solana_pubkey::Pubkey,
    pub resource: solana_pubkey::Pubkey,
    pub planet: solana_pubkey::Pubkey,
    pub fuel_tank: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub pilot_xp_accounts: solana_pubkey::Pubkey,
    pub mining_xp_accounts: solana_pubkey::Pubkey,
    pub council_rank_xp_accounts: solana_pubkey::Pubkey,
    pub progression_config: solana_pubkey::Pubkey,
    pub points_program: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for StopMiningAsteroid {
    type ArrangedAccounts = StopMiningAsteroidInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_accounts_fleet_and_owner = next_account(&mut iter)?;
        let mine_item = next_account(&mut iter)?;
        let resource = next_account(&mut iter)?;
        let planet = next_account(&mut iter)?;
        let fuel_tank = next_account(&mut iter)?;
        let cargo_type = next_account(&mut iter)?;
        let cargo_stats_definition = next_account(&mut iter)?;
        let token_from = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;
        let pilot_xp_accounts = next_account(&mut iter)?;
        let mining_xp_accounts = next_account(&mut iter)?;
        let council_rank_xp_accounts = next_account(&mut iter)?;
        let progression_config = next_account(&mut iter)?;
        let points_program = next_account(&mut iter)?;
        let cargo_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(StopMiningAsteroidInstructionAccounts {
            game_accounts_fleet_and_owner,
            mine_item,
            resource,
            planet,
            fuel_tank,
            cargo_type,
            cargo_stats_definition,
            token_from,
            token_mint,
            pilot_xp_accounts,
            mining_xp_accounts,
            council_rank_xp_accounts,
            progression_config,
            points_program,
            cargo_program,
            token_program,
        })
    }
}
