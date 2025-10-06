use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xadcf65f7ace42769")]
pub struct UpdateShipEscrow {
    pub input: UpdateShipEscrowInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateShipEscrowInstructionAccounts {
    pub old_ship: solana_pubkey::Pubkey,
    pub next: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub game_accounts: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateShipEscrow {
    type ArrangedAccounts = UpdateShipEscrowInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let old_ship = next_account(&mut iter)?;
        let next = next_account(&mut iter)?;
        let starbase_and_starbase_player = next_account(&mut iter)?;
        let game_accounts = next_account(&mut iter)?;

        Some(UpdateShipEscrowInstructionAccounts {
            old_ship,
            next,
            starbase_and_starbase_player,
            game_accounts,
        })
    }
}
