use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4bca01047a6e6694")]
pub struct ApproveProgramLockPrivilege {
    pub bump: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ApproveProgramLockPrivilegeInstructionAccounts {
    pub locker: solana_pubkey::Pubkey,
    pub whitelist_entry: solana_pubkey::Pubkey,
    pub governor: solana_pubkey::Pubkey,
    pub smart_wallet: solana_pubkey::Pubkey,
    pub executable_id: solana_pubkey::Pubkey,
    pub whitelisted_owner: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ApproveProgramLockPrivilege {
    type ArrangedAccounts = ApproveProgramLockPrivilegeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let locker = next_account(&mut iter)?;
        let whitelist_entry = next_account(&mut iter)?;
        let governor = next_account(&mut iter)?;
        let smart_wallet = next_account(&mut iter)?;
        let executable_id = next_account(&mut iter)?;
        let whitelisted_owner = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(ApproveProgramLockPrivilegeInstructionAccounts {
            locker,
            whitelist_entry,
            governor,
            smart_wallet,
            executable_id,
            whitelisted_owner,
            payer,
            system_program,
        })
    }
}
