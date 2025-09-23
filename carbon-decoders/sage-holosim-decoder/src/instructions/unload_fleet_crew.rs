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
    pub fleet_and_owner: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UnloadFleetCrew {
    type ArrangedAccounts = UnloadFleetCrewInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let fleet_and_owner = next_account(&mut iter)?;
        let starbase_and_starbase_player = next_account(&mut iter)?;
        let game_id = next_account(&mut iter)?;

        Some(UnloadFleetCrewInstructionAccounts {
            fleet_and_owner,
            starbase_and_starbase_player,
            game_id,
        })
    }
}
