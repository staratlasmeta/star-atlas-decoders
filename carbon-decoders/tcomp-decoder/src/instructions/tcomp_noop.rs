use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6aa20ae28444df15")]
pub struct TcompNoop {
    pub event: TcompEvent,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TcompNoopInstructionAccounts {
    pub tcomp_signer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TcompNoop {
    type ArrangedAccounts = TcompNoopInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let tcomp_signer = next_account(&mut iter)?;

        Some(TcompNoopInstructionAccounts { tcomp_signer })
    }
}
