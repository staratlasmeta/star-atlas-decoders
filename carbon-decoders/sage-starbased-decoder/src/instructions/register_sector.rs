use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x37393ac024eb246d")]
pub struct RegisterSector {
    pub coordinates: [i64; 2],
    pub name: Vec<u8>,
    pub key_index: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RegisterSectorInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub discoverer: solana_pubkey::Pubkey,
    pub sector: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RegisterSector {
    type ArrangedAccounts = RegisterSectorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_and_profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let discoverer = next_account(&mut iter)?;
        let sector = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RegisterSectorInstructionAccounts {
            game_and_profile,
            funder,
            discoverer,
            sector,
            system_program,
        })
    }
}
