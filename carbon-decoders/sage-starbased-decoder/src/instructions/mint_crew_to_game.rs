use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x40de5ef395413684")]
pub struct MintCrewToGame {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MintCrewToGameInstructionAccounts {
    pub sage_player_profile: solana_pubkey::Pubkey,
    // StarbaseAndStarbasePlayerMut expansion
    pub starbase: solana_pubkey::Pubkey,
    pub starbase_player: solana_pubkey::Pubkey,
    pub sage_crew_config: solana_pubkey::Pubkey,
    pub crew_config: solana_pubkey::Pubkey,
    pub instructions_sysvar: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MintCrewToGame {
    type ArrangedAccounts = MintCrewToGameInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let sage_player_profile = next_account(&mut iter)?;

        // StarbaseAndStarbasePlayerMut expansion
        let starbase = next_account(&mut iter)?;
        let starbase_player = next_account(&mut iter)?;

        let sage_crew_config = next_account(&mut iter)?;
        let crew_config = next_account(&mut iter)?;
        let instructions_sysvar = next_account(&mut iter)?;

        Some(MintCrewToGameInstructionAccounts {
            sage_player_profile,
            starbase,
            starbase_player,
            sage_crew_config,
            crew_config,
            instructions_sysvar,
        })
    }
}
