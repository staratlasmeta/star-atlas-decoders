use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x588eda954b4de49c")]
pub struct RegisterSagePlayerProfile {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RegisterSagePlayerProfileInstructionAccounts {
    pub profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub sage_player_profile: solana_pubkey::Pubkey,
    pub game_accounts: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RegisterSagePlayerProfile {
    type ArrangedAccounts = RegisterSagePlayerProfileInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let sage_player_profile = next_account(&mut iter)?;
        let game_accounts = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RegisterSagePlayerProfileInstructionAccounts {
            profile,
            funder,
            sage_player_profile,
            game_accounts,
            system_program,
        })
    }
}
