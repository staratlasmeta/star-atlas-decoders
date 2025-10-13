
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)] 
 

#[carbon(discriminator = "0xabec8a73f26d24b7")] 
pub struct FeePayerRates {
        pub version: u8,
        pub owning_profile: solana_pubkey::Pubkey,
        pub token_mint: solana_pubkey::Pubkey,
        pub token_owner: solana_pubkey::Pubkey,
        pub token_limit: u64,
        pub conversion_rate: u64, 
}