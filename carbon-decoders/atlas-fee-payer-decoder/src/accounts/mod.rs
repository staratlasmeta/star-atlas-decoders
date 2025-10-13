 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use super::AtlasFeePayerDecoder; 
pub mod fee_payer; 
pub mod fee_payer_rates; 

pub enum AtlasFeePayerAccount { 
        FeePayer(fee_payer::FeePayer), 
        FeePayerRates(fee_payer_rates::FeePayerRates), 
}


impl<'a> AccountDecoder<'a> for AtlasFeePayerDecoder { 
    type AccountType = AtlasFeePayerAccount;
     fn decode_account( &self, account: &solana_account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = fee_payer::FeePayer::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: AtlasFeePayerAccount::FeePayer(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = fee_payer_rates::FeePayerRates::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: AtlasFeePayerAccount::FeePayerRates(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}