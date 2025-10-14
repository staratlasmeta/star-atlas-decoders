use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdd86054c0491ca1d")]
pub struct InitializeOpenOrdersCounter {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeOpenOrdersCounterInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub open_orders_counter: solana_pubkey::Pubkey,
    pub deposit_mint: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeOpenOrdersCounter {
    type ArrangedAccounts = InitializeOpenOrdersCounterInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let payer = next_account(&mut iter)?;
        let user = next_account(&mut iter)?;
        let open_orders_counter = next_account(&mut iter)?;
        let deposit_mint = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(InitializeOpenOrdersCounterInstructionAccounts {
            payer,
            user,
            open_orders_counter,
            deposit_mint,
            system_program,
        })
    }
}
