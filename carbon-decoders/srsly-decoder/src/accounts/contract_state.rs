
use super::super::types::*;
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)] 
 

#[carbon(discriminator = "0xbe8a0adfbd74de73")] 
pub struct ContractState {
        pub version: u8,
        pub to_close: bool,
        pub rate: u64,
        pub duration_min: u64,
        pub duration_max: u64,
        pub payments_feq: PaymentFrequency,
        pub fleet: solana_pubkey::Pubkey,
        pub game_id: solana_pubkey::Pubkey,
        pub current_rental_state: solana_pubkey::Pubkey,
        pub owner: solana_pubkey::Pubkey,
        pub owner_token_account: solana_pubkey::Pubkey,
        pub owner_profile: solana_pubkey::Pubkey,
        pub bump: u8, 
}