use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::PointsDecoder;
pub mod point_category;
pub mod points_modifier;
pub mod user_points_account;

#[derive(Debug, serde::Serialize)]
pub enum PointsAccount {
    PointCategory(point_category::PointCategory),
    PointsModifier(points_modifier::PointsModifier),
    UserPointsAccount(user_points_account::UserPointsAccount),
}

impl<'a> AccountDecoder<'a> for PointsDecoder {
    type AccountType = PointsAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            point_category::PointCategory::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PointsAccount::PointCategory(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            points_modifier::PointsModifier::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PointsAccount::PointsModifier(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            user_points_account::UserPointsAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PointsAccount::UserPointsAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
