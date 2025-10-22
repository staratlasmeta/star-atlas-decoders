use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::ProfileFactionDecoder;
pub mod profile_faction_account;

#[derive(Debug, serde::Serialize)]
pub enum ProfileFactionAccount {
    ProfileFactionAccount(profile_faction_account::ProfileFactionAccount),
}

impl<'a> AccountDecoder<'a> for ProfileFactionDecoder {
    type AccountType = ProfileFactionAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            profile_faction_account::ProfileFactionAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ProfileFactionAccount::ProfileFactionAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
