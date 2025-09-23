use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x28064500e696d729")]
pub struct AddConnection {
    pub sub_coordinates1: [i64; 2],
    pub flags1: u8,
    pub sub_coordinates2: [i64; 2],
    pub flags2: u8,
    pub key_index: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AddConnectionInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub sector1: solana_pubkey::Pubkey,
    pub sector2: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddConnection {
    type ArrangedAccounts = AddConnectionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_and_profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let sector1 = next_account(&mut iter)?;
        let sector2 = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(AddConnectionInstructionAccounts {
            game_and_profile,
            funder,
            sector1,
            sector2,
            system_program,
        })
    }
}
