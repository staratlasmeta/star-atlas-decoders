use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x12a3f263760ec756")]
pub struct ProcessInitializeBuyPda {
    pub price: u64,
    pub origination_qty: u64,
    pub order_account_pubkey: solana_pubkey::Pubkey,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ProcessInitializeBuyPdaInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub order_initializer: solana_pubkey::Pubkey,
    pub market_vars_account: solana_pubkey::Pubkey,
    pub deposit_mint: solana_pubkey::Pubkey,
    pub receive_mint: solana_pubkey::Pubkey,
    pub order_vault_account: solana_pubkey::Pubkey,
    pub order_vault_authority: solana_pubkey::Pubkey,
    pub initializer_deposit_token_account: solana_pubkey::Pubkey,
    pub initializer_receive_token_account: solana_pubkey::Pubkey,
    pub order_account: solana_pubkey::Pubkey,
    pub registered_currency: solana_pubkey::Pubkey,
    pub open_orders_counter: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ProcessInitializeBuyPda {
    type ArrangedAccounts = ProcessInitializeBuyPdaInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let payer = next_account(&mut iter)?;
        let order_initializer = next_account(&mut iter)?;
        let market_vars_account = next_account(&mut iter)?;
        let deposit_mint = next_account(&mut iter)?;
        let receive_mint = next_account(&mut iter)?;
        let order_vault_account = next_account(&mut iter)?;
        let order_vault_authority = next_account(&mut iter)?;
        let initializer_deposit_token_account = next_account(&mut iter)?;
        let initializer_receive_token_account = next_account(&mut iter)?;
        let order_account = next_account(&mut iter)?;
        let registered_currency = next_account(&mut iter)?;
        let open_orders_counter = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(ProcessInitializeBuyPdaInstructionAccounts {
            payer,
            order_initializer,
            market_vars_account,
            deposit_mint,
            receive_mint,
            order_vault_account,
            order_vault_authority,
            initializer_deposit_token_account,
            initializer_receive_token_account,
            order_account,
            registered_currency,
            open_orders_counter,
            system_program,
            rent,
            token_program,
        })
    }
}
