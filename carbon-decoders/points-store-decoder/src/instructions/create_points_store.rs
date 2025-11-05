use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xcd926983fea93a49")]
pub struct CreatePointsStore {
    pub input: CreatePointsStoreInputUnpacked,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreatePointsStoreInstructionAccounts {
    pub profile: solana_pubkey::Pubkey,
    pub store: solana_pubkey::Pubkey,
    pub store_signer: solana_pubkey::Pubkey,
    pub point_category: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub bank: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreatePointsStore {
    type ArrangedAccounts = CreatePointsStoreInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let profile = next_account(&mut iter)?;
        let store = next_account(&mut iter)?;
        let store_signer = next_account(&mut iter)?;
        let point_category = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let bank = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CreatePointsStoreInstructionAccounts {
            profile,
            store,
            store_signer,
            point_category,
            funder,
            bank,
            system_program,
        })
    }
}
