use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::ProfileVaultDecoder;
pub mod vault_authority;

pub enum ProfileVaultAccount {
    VaultAuthority(vault_authority::VaultAuthority),
}

impl<'a> AccountDecoder<'a> for ProfileVaultDecoder {
    type AccountType = ProfileVaultAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            vault_authority::VaultAuthority::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ProfileVaultAccount::VaultAuthority(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
