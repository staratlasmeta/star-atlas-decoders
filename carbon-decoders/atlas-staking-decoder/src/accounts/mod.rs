use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::AtlasStakingDecoder;
pub mod registered_stake;
pub mod staking_account;
pub mod staking_vars;

pub enum AtlasStakingAccount {
    RegisteredStake(registered_stake::RegisteredStake),
    StakingAccount(staking_account::StakingAccount),
    StakingVars(staking_vars::StakingVars),
}

impl<'a> AccountDecoder<'a> for AtlasStakingDecoder {
    type AccountType = AtlasStakingAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            registered_stake::RegisteredStake::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: AtlasStakingAccount::RegisteredStake(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            staking_account::StakingAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: AtlasStakingAccount::StakingAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            staking_vars::StakingVars::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: AtlasStakingAccount::StakingVars(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
