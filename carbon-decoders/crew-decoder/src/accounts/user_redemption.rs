
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)] 
 

#[carbon(discriminator = "0x40a650064e968ff9")] 
pub struct UserRedemption {
        pub version: u8,
        pub bump: u8,
        pub crew_config: solana_pubkey::Pubkey,
        pub seed_pubkey: solana_pubkey::Pubkey,
        pub owner: solana_pubkey::Pubkey,
        pub mint_offset: u32,
        pub amount: u32,
        pub number_minted: u32,
        pub pack_type: solana_pubkey::Pubkey,
        pub user_seed: [u8; 32],
        pub server_hash: [u8; 32],
        pub sage_profile: solana_pubkey::Pubkey, 
}