use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xca15e19c0f046a5d")]
pub struct CloseCraftingProcess {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseCraftingProcessInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub crafting_process: solana_pubkey::Pubkey,
    pub recipe: solana_pubkey::Pubkey,
    pub crafting_facility: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseCraftingProcess {
    type ArrangedAccounts = CloseCraftingProcessInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let authority = next_account(&mut iter)?;
        let funds_to = next_account(&mut iter)?;
        let crafting_process = next_account(&mut iter)?;
        let recipe = next_account(&mut iter)?;
        let crafting_facility = next_account(&mut iter)?;

        Some(CloseCraftingProcessInstructionAccounts {
            authority,
            funds_to,
            crafting_process,
            recipe,
            crafting_facility,
        })
    }
}
