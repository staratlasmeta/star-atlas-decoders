use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x52a34eeda53b36b2")]
pub struct RegisterPointModifier {
    pub can_increment: bool,
    pub can_decrement: bool,
    pub key_index: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RegisterPointModifierInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub category: solana_pubkey::Pubkey,
    pub points_modifier_account: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RegisterPointModifier {
    type ArrangedAccounts = RegisterPointModifierInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let category = next_account(&mut iter)?;
        let points_modifier_account = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(RegisterPointModifierInstructionAccounts {
            key,
            profile,
            funder,
            category,
            points_modifier_account,
            system_program,
        })
    }
}
