use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3c129e13d09353e2")]
pub struct RegisterStarbasePlayer {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RegisterStarbasePlayerInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub game_accounts: solana_pubkey::Pubkey,
    pub sage_player_profile: solana_pubkey::Pubkey,
    pub profile_faction: solana_pubkey::Pubkey,
    pub starbase: solana_pubkey::Pubkey,
    pub starbase_player: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RegisterStarbasePlayer {
    type ArrangedAccounts = RegisterStarbasePlayerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funder = next_account(&mut iter)?;
        let game_accounts = next_account(&mut iter)?;
        let sage_player_profile = next_account(&mut iter)?;
        let profile_faction = next_account(&mut iter)?;
        let starbase = next_account(&mut iter)?;
        let starbase_player = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RegisterStarbasePlayerInstructionAccounts {
            funder,
            game_accounts,
            sage_player_profile,
            profile_faction,
            starbase,
            starbase_player,
            system_program,
        })
    }
}
