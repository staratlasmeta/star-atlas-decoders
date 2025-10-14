use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x70c23f6334935530")]
pub struct ProcessExchange {
    pub purchase_quantity: u64,
    pub expected_price: u64,
    pub seller: solana_pubkey::Pubkey,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ProcessExchangeInstructionAccounts {
    pub order_taker: solana_pubkey::Pubkey,
    pub order_taker_deposit_token_account: solana_pubkey::Pubkey,
    pub order_taker_receive_token_account: solana_pubkey::Pubkey,
    pub currency_mint: solana_pubkey::Pubkey,
    pub asset_mint: solana_pubkey::Pubkey,
    pub order_initializer: solana_pubkey::Pubkey,
    pub initializer_deposit_token_account: solana_pubkey::Pubkey,
    pub initializer_receive_token_account: solana_pubkey::Pubkey,
    pub order_vault_account: solana_pubkey::Pubkey,
    pub order_vault_authority: solana_pubkey::Pubkey,
    pub order_account: solana_pubkey::Pubkey,
    pub sa_vault: solana_pubkey::Pubkey,
    pub registered_currency: solana_pubkey::Pubkey,
    pub open_orders_counter: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub atlas_staking: solana_pubkey::Pubkey,
    pub registered_stake: solana_pubkey::Pubkey,
    pub staking_account: solana_pubkey::Pubkey,
    pub fee_reduction: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ProcessExchange {
    type ArrangedAccounts = ProcessExchangeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let order_taker = next_account(&mut iter)?;
        let order_taker_deposit_token_account = next_account(&mut iter)?;
        let order_taker_receive_token_account = next_account(&mut iter)?;
        let currency_mint = next_account(&mut iter)?;
        let asset_mint = next_account(&mut iter)?;
        let order_initializer = next_account(&mut iter)?;
        let initializer_deposit_token_account = next_account(&mut iter)?;
        let initializer_receive_token_account = next_account(&mut iter)?;
        let order_vault_account = next_account(&mut iter)?;
        let order_vault_authority = next_account(&mut iter)?;
        let order_account = next_account(&mut iter)?;
        let sa_vault = next_account(&mut iter)?;
        let registered_currency = next_account(&mut iter)?;
        let open_orders_counter = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let atlas_staking = next_account(&mut iter)?;
        let registered_stake = next_account(&mut iter)?;
        let staking_account = next_account(&mut iter)?;
        let fee_reduction = next_account(&mut iter)?;

        Some(ProcessExchangeInstructionAccounts {
            order_taker,
            order_taker_deposit_token_account,
            order_taker_receive_token_account,
            currency_mint,
            asset_mint,
            order_initializer,
            initializer_deposit_token_account,
            initializer_receive_token_account,
            order_vault_account,
            order_vault_authority,
            order_account,
            sa_vault,
            registered_currency,
            open_orders_counter,
            token_program,
            atlas_staking,
            registered_stake,
            staking_account,
            fee_reduction,
        })
    }
}
