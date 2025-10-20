use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x990ecb4fcd451bcf")]
pub struct RemoveStoreItems {
    pub amount: u64,
    pub key_index: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RemoveStoreItemsInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub store: solana_pubkey::Pubkey,
    pub store_signer: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
    pub tokens_to: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveStoreItems {
    type ArrangedAccounts = RemoveStoreItemsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let key = next_account(&mut iter)?;
        let profile = next_account(&mut iter)?;
        let store = next_account(&mut iter)?;
        let store_signer = next_account(&mut iter)?;
        let bank = next_account(&mut iter)?;
        let tokens_to = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(RemoveStoreItemsInstructionAccounts {
            key,
            profile,
            store,
            store_signer,
            bank,
            tokens_to,
            token_program,
        })
    }
}
