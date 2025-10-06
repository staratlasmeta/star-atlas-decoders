use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x59ad057b6d0decd9")]
pub struct AttackFleet {
    pub input: AttackFleetInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AttackFleetInstructionAccounts {
    pub game_and_fleet_and_owner: solana_pubkey::Pubkey,
    pub defending_fleet: solana_pubkey::Pubkey,
    pub attacking_cargo_pod: solana_pubkey::Pubkey,
    pub defending_cargo_pod: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub attacker_combat_xp: solana_pubkey::Pubkey,
    pub attacker_council_rank_xp: solana_pubkey::Pubkey,
    pub defender_combat_xp: solana_pubkey::Pubkey,
    pub defender_council_rank_xp: solana_pubkey::Pubkey,
    pub combat_xp_category: solana_pubkey::Pubkey,
    pub council_rank_xp_category: solana_pubkey::Pubkey,
    pub combat_xp_modifier: solana_pubkey::Pubkey,
    pub council_rank_xp_modifier: solana_pubkey::Pubkey,
    pub progression_config: solana_pubkey::Pubkey,
    pub combat_config: solana_pubkey::Pubkey,
    pub attacking_fleet_ammo_token: solana_pubkey::Pubkey,
    pub defending_fleet_ammo_token: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AttackFleet {
    type ArrangedAccounts = AttackFleetInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_and_fleet_and_owner = next_account(&mut iter)?;
        let defending_fleet = next_account(&mut iter)?;
        let attacking_cargo_pod = next_account(&mut iter)?;
        let defending_cargo_pod = next_account(&mut iter)?;
        let cargo_type = next_account(&mut iter)?;
        let cargo_stats_definition = next_account(&mut iter)?;
        let attacker_combat_xp = next_account(&mut iter)?;
        let attacker_council_rank_xp = next_account(&mut iter)?;
        let defender_combat_xp = next_account(&mut iter)?;
        let defender_council_rank_xp = next_account(&mut iter)?;
        let combat_xp_category = next_account(&mut iter)?;
        let council_rank_xp_category = next_account(&mut iter)?;
        let combat_xp_modifier = next_account(&mut iter)?;
        let council_rank_xp_modifier = next_account(&mut iter)?;
        let progression_config = next_account(&mut iter)?;
        let combat_config = next_account(&mut iter)?;
        let attacking_fleet_ammo_token = next_account(&mut iter)?;
        let defending_fleet_ammo_token = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;

        Some(AttackFleetInstructionAccounts {
            game_and_fleet_and_owner,
            defending_fleet,
            attacking_cargo_pod,
            defending_cargo_pod,
            cargo_type,
            cargo_stats_definition,
            attacker_combat_xp,
            attacker_council_rank_xp,
            defender_combat_xp,
            defender_council_rank_xp,
            combat_xp_category,
            council_rank_xp_category,
            combat_xp_modifier,
            council_rank_xp_modifier,
            progression_config,
            combat_config,
            attacking_fleet_ammo_token,
            defending_fleet_ammo_token,
            token_mint,
        })
    }
}
