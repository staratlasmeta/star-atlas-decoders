
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)] 
 

#[carbon(discriminator = "0x8422bbcacac3d335")] 
pub struct VaultAuthority {
        pub version: u8,
        pub profile: solana_pubkey::Pubkey,
        pub vault_seed: solana_pubkey::Pubkey,
        pub vault_bump: u8, 
}