use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc891776755be788a")]
pub struct RemoveConnection {
    pub sector1_index: u16,
    pub sector2_index: u16,
    pub key_index: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RemoveConnectionInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub sector1: solana_pubkey::Pubkey,
    pub sector2: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveConnection {
    type ArrangedAccounts = RemoveConnectionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let game_and_profile = next_account(&mut iter)?;
        let funds_to = next_account(&mut iter)?;
        let sector1 = next_account(&mut iter)?;
        let sector2 = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RemoveConnectionInstructionAccounts {
            game_and_profile,
            funds_to,
            sector1,
            sector2,
            system_program,
        })
    }
}
