use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::CargoDecoder;
pub mod cargo_pod;
pub mod cargo_stats_definition;
pub mod cargo_type;

#[derive(Debug, serde::Serialize)]
pub enum CargoAccount {
    CargoPod(cargo_pod::CargoPod),
    CargoStatsDefinition(cargo_stats_definition::CargoStatsDefinition),
    CargoType(cargo_type::CargoType),
}

impl<'a> AccountDecoder<'a> for CargoDecoder {
    type AccountType = CargoAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) = cargo_pod::CargoPod::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: CargoAccount::CargoPod(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            cargo_stats_definition::CargoStatsDefinition::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: CargoAccount::CargoStatsDefinition(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = cargo_type::CargoType::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: CargoAccount::CargoType(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
