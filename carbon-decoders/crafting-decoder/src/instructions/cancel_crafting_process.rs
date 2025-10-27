use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd71e81805be7f94e")]
pub struct CancelCraftingProcess {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CancelCraftingProcessInstructionAccounts {
    pub location: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub crafting_process: solana_pubkey::Pubkey,
    pub crafting_facility: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelCraftingProcess {
    type ArrangedAccounts = CancelCraftingProcessInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let location = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let funds_to = next_account(&mut iter)?;
        let crafting_process = next_account(&mut iter)?;
        let crafting_facility = next_account(&mut iter)?;

        Some(CancelCraftingProcessInstructionAccounts {
            location,
            authority,
            funds_to,
            crafting_process,
            crafting_facility,
        })
    }
}
