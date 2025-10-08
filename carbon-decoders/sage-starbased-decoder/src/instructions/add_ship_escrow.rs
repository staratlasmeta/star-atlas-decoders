use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xba13da96a7b5d459")]
pub struct AddShipEscrow {
    pub input: AddShipEscrowInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AddShipEscrowInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub sage_player_profile: solana_pubkey::Pubkey,
    pub origin_token_account: solana_pubkey::Pubkey,
    pub ship: solana_pubkey::Pubkey,
    pub ship_escrow_token_account: solana_pubkey::Pubkey,
    // StarbaseAndStarbasePlayerMut expansion
    pub starbase: solana_pubkey::Pubkey,
    pub starbase_player: solana_pubkey::Pubkey,
    // GameAndGameStateAndProfile expansion
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub profile_faction: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub game_state: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddShipEscrow {
    type ArrangedAccounts = AddShipEscrowInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funder = next_account(&mut iter)?;
        let sage_player_profile = next_account(&mut iter)?;
        let origin_token_account = next_account(&mut iter)?;
        let ship = next_account(&mut iter)?;
        let ship_escrow_token_account = next_account(&mut iter)?;

        // StarbaseAndStarbasePlayerMut expansion
        let starbase = next_account(&mut iter)?;
        let starbase_player = next_account(&mut iter)?;

        // GameAndGameStateAndProfile expansion
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let profile_faction = next_account(&mut iter)?;
        let game_id = next_account(&mut iter)?;
        let game_state = next_account(&mut iter)?;

        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(AddShipEscrowInstructionAccounts {
            funder,
            sage_player_profile,
            origin_token_account,
            ship,
            ship_escrow_token_account,
            starbase,
            starbase_player,
            key,
            profile,
            profile_faction,
            game_id,
            game_state,
            token_program,
            system_program,
        })
    }
}
