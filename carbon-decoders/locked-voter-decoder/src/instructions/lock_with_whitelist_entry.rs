use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8af8b94f03737339")]
pub struct LockWithWhitelistEntry {
    pub amount: u64,
    pub duration: i64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct LockWithWhitelistEntryInstructionAccounts {
    pub lock: solana_pubkey::Pubkey,
    pub instructions_sysvar: solana_pubkey::Pubkey,
    pub whitelist_entry: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LockWithWhitelistEntry {
    type ArrangedAccounts = LockWithWhitelistEntryInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let lock = next_account(&mut iter)?;
        let instructions_sysvar = next_account(&mut iter)?;
        let whitelist_entry = next_account(&mut iter)?;

        Some(LockWithWhitelistEntryInstructionAccounts {
            lock,
            instructions_sysvar,
            whitelist_entry,
        })
    }
}
