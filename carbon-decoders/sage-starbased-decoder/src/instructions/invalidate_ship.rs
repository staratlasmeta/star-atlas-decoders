use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x08077356f6435794")]
pub struct InvalidateShip {
    pub key_index: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InvalidateShipInstructionAccounts {
    // ActiveOrInactiveGameAndProfileMut expansion
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub ship: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InvalidateShip {
    type ArrangedAccounts = InvalidateShipInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        // ActiveOrInactiveGameAndProfileMut expansion
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let game_id = next_account(&mut iter)?;

        let ship = next_account(&mut iter)?;

        Some(InvalidateShipInstructionAccounts {
            key,
            profile,
            game_id,
            ship,
        })
    }
}
