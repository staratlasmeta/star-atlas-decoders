use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5712d5c03f8841cd")]
pub struct CloseStore {
    pub key_index: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseStoreInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub store: solana_pubkey::Pubkey,
    pub store_signer: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub tokens_to: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseStore {
    type ArrangedAccounts = CloseStoreInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let store = next_account(&mut iter)?;
        let store_signer = next_account(&mut iter)?;
        let bank = next_account(&mut iter)?;
        let funds_to = next_account(&mut iter)?;
        let tokens_to = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CloseStoreInstructionAccounts {
            key,
            profile,
            store,
            store_signer,
            bank,
            funds_to,
            tokens_to,
            token_program,
            system_program,
        })
    }
}
