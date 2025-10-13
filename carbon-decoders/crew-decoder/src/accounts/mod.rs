 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use super::CrewDecoder; 
pub mod crew_config; 
pub mod pack_tiers; 
pub mod pack_type; 
pub mod sft_redemption; 
pub mod user_redemption; 

pub enum CrewAccount { 
        CrewConfig(crew_config::CrewConfig), 
        PackTiers(pack_tiers::PackTiers), 
        PackType(pack_type::PackType), 
        SftRedemption(sft_redemption::SftRedemption), 
        UserRedemption(user_redemption::UserRedemption), 
}


impl<'a> AccountDecoder<'a> for CrewDecoder { 
    type AccountType = CrewAccount;
     fn decode_account( &self, account: &solana_account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = crew_config::CrewConfig::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: CrewAccount::CrewConfig(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = pack_tiers::PackTiers::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: CrewAccount::PackTiers(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = pack_type::PackType::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: CrewAccount::PackType(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = sft_redemption::SftRedemption::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: CrewAccount::SftRedemption(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = user_redemption::UserRedemption::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: CrewAccount::UserRedemption(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}