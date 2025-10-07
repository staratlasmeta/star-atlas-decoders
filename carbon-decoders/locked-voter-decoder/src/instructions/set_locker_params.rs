use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6a278454fe4da1a9")]
pub struct SetLockerParams {
    pub params: LockerParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetLockerParamsInstructionAccounts {
    pub locker: solana_pubkey::Pubkey,
    pub governor: solana_pubkey::Pubkey,
    pub smart_wallet: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetLockerParams {
    type ArrangedAccounts = SetLockerParamsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let locker = next_account(&mut iter)?;
        let governor = next_account(&mut iter)?;
        let smart_wallet = next_account(&mut iter)?;

        Some(SetLockerParamsInstructionAccounts {
            locker,
            governor,
            smart_wallet,
        })
    }
}
