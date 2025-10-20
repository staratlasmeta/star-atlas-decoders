use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::PointsStoreDecoder;
pub mod points_store;
pub mod redemption_config;
pub mod user_redemption;

#[derive(Debug, serde::Serialize)]
pub enum PointsStoreAccount {
    PointsStore(points_store::PointsStore),
    RedemptionConfig(redemption_config::RedemptionConfig),
    UserRedemption(user_redemption::UserRedemption),
}

impl<'a> AccountDecoder<'a> for PointsStoreDecoder {
    type AccountType = PointsStoreAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            points_store::PointsStore::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PointsStoreAccount::PointsStore(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            redemption_config::RedemptionConfig::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PointsStoreAccount::RedemptionConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            user_redemption::UserRedemption::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PointsStoreAccount::UserRedemption(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
