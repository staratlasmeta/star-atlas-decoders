
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)] 
 

#[carbon(discriminator = "0x03fc32a24c2e90d5")] 
pub struct FeePayer {
        pub version: u8,
        pub rates: solana_pubkey::Pubkey,
        pub token_vault: solana_pubkey::Pubkey,
        pub payment_account: solana_pubkey::Pubkey,
        pub last_payer_value: u64, 
}