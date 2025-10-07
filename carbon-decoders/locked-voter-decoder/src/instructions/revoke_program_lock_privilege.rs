use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xaa970758c256f570")]
pub struct RevokeProgramLockPrivilege {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RevokeProgramLockPrivilegeInstructionAccounts {
    pub locker: solana_pubkey::Pubkey,
    pub whitelist_entry: solana_pubkey::Pubkey,
    pub governor: solana_pubkey::Pubkey,
    pub smart_wallet: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RevokeProgramLockPrivilege {
    type ArrangedAccounts = RevokeProgramLockPrivilegeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let locker = next_account(&mut iter)?;
        let whitelist_entry = next_account(&mut iter)?;
        let governor = next_account(&mut iter)?;
        let smart_wallet = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;

        Some(RevokeProgramLockPrivilegeInstructionAccounts {
            locker,
            whitelist_entry,
            governor,
            smart_wallet,
            payer,
        })
    }
}
