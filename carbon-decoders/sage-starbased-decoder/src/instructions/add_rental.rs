use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd571aa7bbb5a1c73")]
pub struct AddRental {
    pub owner_key_index: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AddRentalInstructionAccounts {
    pub owner_profile: solana_pubkey::Pubkey,
    pub owner_key: solana_pubkey::Pubkey,
    pub invalidator: solana_pubkey::Pubkey,
    pub fleet: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddRental {
    type ArrangedAccounts = AddRentalInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let owner_profile = next_account(&mut iter)?;
        let owner_key = next_account(&mut iter)?;
        let invalidator = next_account(&mut iter)?;
        let fleet = next_account(&mut iter)?;
        let game_id = next_account(&mut iter)?;

        Some(AddRentalInstructionAccounts {
            owner_profile,
            owner_key,
            invalidator,
            fleet,
            game_id,
        })
    }
}
