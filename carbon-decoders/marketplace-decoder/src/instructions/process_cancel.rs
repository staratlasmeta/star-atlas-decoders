use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5554d6f08c29e695")]
pub struct ProcessCancel {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ProcessCancelInstructionAccounts {
    pub signer: solana_pubkey::Pubkey,
    pub order_initializer: solana_pubkey::Pubkey,
    pub market_vars_account: solana_pubkey::Pubkey,
    pub deposit_mint: solana_pubkey::Pubkey,
    pub initializer_deposit_token_account: solana_pubkey::Pubkey,
    pub order_vault_account: solana_pubkey::Pubkey,
    pub order_vault_authority: solana_pubkey::Pubkey,
    pub order_account: solana_pubkey::Pubkey,
    pub open_orders_counter: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ProcessCancel {
    type ArrangedAccounts = ProcessCancelInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let signer = next_account(&mut iter)?;
        let order_initializer = next_account(&mut iter)?;
        let market_vars_account = next_account(&mut iter)?;
        let deposit_mint = next_account(&mut iter)?;
        let initializer_deposit_token_account = next_account(&mut iter)?;
        let order_vault_account = next_account(&mut iter)?;
        let order_vault_authority = next_account(&mut iter)?;
        let order_account = next_account(&mut iter)?;
        let open_orders_counter = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(ProcessCancelInstructionAccounts {
            signer,
            order_initializer,
            market_vars_account,
            deposit_mint,
            initializer_deposit_token_account,
            order_vault_account,
            order_vault_authority,
            order_account,
            open_orders_counter,
            token_program,
        })
    }
}
