
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)] 
 

#[carbon(discriminator = "0x4ef2598aa1ddb04b")] 
pub struct ListState {
        pub version: u8,
        pub bump: [u8; 1],
        pub owner: solana_pubkey::Pubkey,
        pub asset_id: solana_pubkey::Pubkey,
        pub amount: u64,
        pub currency: Option<solana_pubkey::Pubkey>,
        pub expiry: i64,
        pub private_taker: Option<solana_pubkey::Pubkey>,
        pub maker_broker: Option<solana_pubkey::Pubkey>,
        pub rent_payer: solana_pubkey::Pubkey,
        pub reserved: [u8; 32],
        #[serde(with = "serde_big_array::BigArray")]
        pub reserved1: [u8; 64], 
}