
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)] 
 

#[carbon(discriminator = "0x61a21ddefbfbb4f4")] 
pub struct RentalState {
        pub version: u8,
        pub borrower: solana_pubkey::Pubkey,
        pub thread: solana_pubkey::Pubkey,
        pub contract: solana_pubkey::Pubkey,
        pub owner_token_account: solana_pubkey::Pubkey,
        pub rate: f64,
        pub start_time: i64,
        pub end_time: i64,
        pub cancelled: bool,
        pub bump: u8, 
}