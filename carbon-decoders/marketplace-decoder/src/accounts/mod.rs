use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::MarketplaceDecoder;
pub mod atlas_rate_account;
pub mod fee_reduction;
pub mod market_vars;
pub mod open_orders_counter;
pub mod order_account;
pub mod registered_currency;

#[derive(Debug, serde::Serialize)]
pub enum MarketplaceAccount {
    AtlasRateAccount(atlas_rate_account::AtlasRateAccount),
    FeeReduction(fee_reduction::FeeReduction),
    MarketVars(market_vars::MarketVars),
    OpenOrdersCounter(open_orders_counter::OpenOrdersCounter),
    OrderAccount(order_account::OrderAccount),
    RegisteredCurrency(registered_currency::RegisteredCurrency),
}

impl<'a> AccountDecoder<'a> for MarketplaceDecoder {
    type AccountType = MarketplaceAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            atlas_rate_account::AtlasRateAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MarketplaceAccount::AtlasRateAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            fee_reduction::FeeReduction::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MarketplaceAccount::FeeReduction(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = market_vars::MarketVars::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MarketplaceAccount::MarketVars(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            open_orders_counter::OpenOrdersCounter::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MarketplaceAccount::OpenOrdersCounter(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            order_account::OrderAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MarketplaceAccount::OrderAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            registered_currency::RegisteredCurrency::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MarketplaceAccount::RegisteredCurrency(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
