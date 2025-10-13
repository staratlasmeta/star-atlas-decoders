
use super::super::types::*;
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)] 
 

#[carbon(discriminator = "0xfe32627ca6f1c4ae")] 
pub struct CrewConfig {
        pub version: u8,
        pub seed_pubkey: solana_pubkey::Pubkey,
        pub bump: u8,
        pub profile: solana_pubkey::Pubkey,
        pub name_prefix: [u8; 32],
        pub name_prefix_len: u8,
        pub symbol: [u8; 10],
        pub symbol_len: u8,
        #[serde(with = "serde_big_array::BigArray")]
        pub uri_prefix: [u8; 40],
        pub uri_prefix_len: u8,
        pub seller_fee_basis_points: u16,
        pub collection_mint: solana_pubkey::Pubkey,
        pub creators: [CrewCreator; 4],
        pub creator_count: u8,
        pub total_minted: u32,
        pub total_allocated: u32, 
}