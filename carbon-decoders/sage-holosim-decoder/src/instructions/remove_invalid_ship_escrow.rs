use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf80f10c50df82700")]
pub struct RemoveInvalidShipEscrow {
    pub input: RemoveShipEscrowInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RemoveInvalidShipEscrowInstructionAccounts {
    pub game_accounts_and_profile: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub sage_player_profile: solana_pubkey::Pubkey,
    pub destination_token_account: solana_pubkey::Pubkey,
    pub ship: solana_pubkey::Pubkey,
    pub ship_escrow_token_account: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveInvalidShipEscrow {
    type ArrangedAccounts = RemoveInvalidShipEscrowInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_accounts_and_profile = next_account(&mut iter)?;
        let funds_to = next_account(&mut iter)?;
        let sage_player_profile = next_account(&mut iter)?;
        let destination_token_account = next_account(&mut iter)?;
        let ship = next_account(&mut iter)?;
        let ship_escrow_token_account = next_account(&mut iter)?;
        let starbase_and_starbase_player = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RemoveInvalidShipEscrowInstructionAccounts {
            game_accounts_and_profile,
            funds_to,
            sage_player_profile,
            destination_token_account,
            ship,
            ship_escrow_token_account,
            starbase_and_starbase_player,
            token_program,
            system_program,
        })
    }
}
