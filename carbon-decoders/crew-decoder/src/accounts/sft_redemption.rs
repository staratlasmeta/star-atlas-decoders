
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)] 
 

#[carbon(discriminator = "0x3a5a5431a80ec3be")] 
pub struct SftRedemption {
        pub version: u8,
        pub bump: u8,
        pub pack_type: solana_pubkey::Pubkey,
        pub sft_mint: solana_pubkey::Pubkey,
        pub crew_config: solana_pubkey::Pubkey,
        pub redemption_amount: u32, 
}