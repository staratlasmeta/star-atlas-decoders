use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x675f2a554f904720")]
pub struct UnloadFleetCrew {
    pub input: FleetCrewInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UnloadFleetCrewInstructionAccounts {
    // FleetAndOwner expansion
    pub key: solana_pubkey::Pubkey,
    pub owning_profile: solana_pubkey::Pubkey,
    pub owning_profile_faction: solana_pubkey::Pubkey,
    pub fleet: solana_pubkey::Pubkey,
    // StarbaseMutAndStarbasePlayer expansion
    pub starbase: solana_pubkey::Pubkey,
    pub starbase_player: solana_pubkey::Pubkey,
    // Direct accounts
    pub game_id: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UnloadFleetCrew {
    type ArrangedAccounts = UnloadFleetCrewInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        // FleetAndOwner expansion
        let key = next_account(&mut iter)?;
        let owning_profile = next_account(&mut iter)?;
        let owning_profile_faction = next_account(&mut iter)?;
        let fleet = next_account(&mut iter)?;
        // StarbaseMutAndStarbasePlayer expansion
        let starbase = next_account(&mut iter)?;
        let starbase_player = next_account(&mut iter)?;
        // Direct accounts
        let game_id = next_account(&mut iter)?;

        Some(UnloadFleetCrewInstructionAccounts {
            key,
            owning_profile,
            owning_profile_faction,
            fleet,
            starbase,
            starbase_player,
            game_id,
        })
    }
}
