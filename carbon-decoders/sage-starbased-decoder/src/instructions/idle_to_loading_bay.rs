use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe32e044a97f82983")]
pub struct IdleToLoadingBay {
    pub key_index: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct IdleToLoadingBayInstructionAccounts {
    pub game_accounts_fleet_and_owner: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for IdleToLoadingBay {
    type ArrangedAccounts = IdleToLoadingBayInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_accounts_fleet_and_owner = next_account(&mut iter)?;
        let starbase_and_starbase_player = next_account(&mut iter)?;

        Some(IdleToLoadingBayInstructionAccounts {
            game_accounts_fleet_and_owner,
            starbase_and_starbase_player,
        })
    }
}
