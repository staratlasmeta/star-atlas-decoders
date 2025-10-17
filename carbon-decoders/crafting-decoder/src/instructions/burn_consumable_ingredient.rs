use super::super::types::*;

use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x041231a72489ea3b")]
pub struct BurnConsumableIngredient {
    pub input: BurnConsumableIngredientInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct BurnConsumableIngredientInstructionAccounts {
    pub crafting_process: solana_pubkey::Pubkey,
    pub recipe: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BurnConsumableIngredient {
    type ArrangedAccounts = BurnConsumableIngredientInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let crafting_process = next_account(&mut iter)?;
        let recipe = next_account(&mut iter)?;
        let token_from = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(BurnConsumableIngredientInstructionAccounts {
            crafting_process,
            recipe,
            token_from,
            mint,
            token_program,
        })
    }
}
