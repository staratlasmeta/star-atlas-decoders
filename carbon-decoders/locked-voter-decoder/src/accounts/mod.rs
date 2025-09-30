use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::LockedVoterDecoder;
pub mod escrow;
pub mod locker;
pub mod locker_whitelist_entry;

pub enum LockedVoterAccount {
    Locker(locker::Locker),
    LockerWhitelistEntry(locker_whitelist_entry::LockerWhitelistEntry),
    Escrow(escrow::Escrow),
}

impl<'a> AccountDecoder<'a> for LockedVoterDecoder {
    type AccountType = LockedVoterAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) = locker::Locker::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: LockedVoterAccount::Locker(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            locker_whitelist_entry::LockerWhitelistEntry::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: LockedVoterAccount::LockerWhitelistEntry(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = escrow::Escrow::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: LockedVoterAccount::Escrow(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
