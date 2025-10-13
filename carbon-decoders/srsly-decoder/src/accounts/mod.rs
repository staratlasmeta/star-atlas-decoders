 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use super::SrslyDecoder; 
pub mod contract_state; 
pub mod fleet; 
pub mod rental_state; 
pub mod thread; 

pub enum SrslyAccount { 
        ContractState(contract_state::ContractState), 
        Fleet(fleet::Fleet), 
        RentalState(rental_state::RentalState), 
        Thread(thread::Thread), 
}


impl<'a> AccountDecoder<'a> for SrslyDecoder { 
    type AccountType = SrslyAccount;
     fn decode_account( &self, account: &solana_account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = contract_state::ContractState::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: SrslyAccount::ContractState(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = fleet::Fleet::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: SrslyAccount::Fleet(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = rental_state::RentalState::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: SrslyAccount::RentalState(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = thread::Thread::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: SrslyAccount::Thread(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}