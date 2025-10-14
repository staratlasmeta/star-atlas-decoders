
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)] 
 

#[carbon(discriminator = "0xa967c89e34fc182b")] 
pub struct PackTiers {
        pub version: u8,
        pub crew_config: solana_pubkey::Pubkey,
        pub seed_pubkey: solana_pubkey::Pubkey,
        pub tier: u8,
        pub bump: u8,
        pub common: u32,
        pub uncommon: u32,
        pub rare: u32,
        pub epic: u32,
        pub legendary: u32,
        pub anomaly: u32, 
}