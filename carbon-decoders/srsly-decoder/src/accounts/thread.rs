
use super::super::types::*;
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)] 
 

#[carbon(discriminator = "0xba1b9a6f33249f5a")] 
pub struct Thread {
        pub authority: solana_pubkey::Pubkey,
        pub bump: u8,
        pub created_at: ClockData,
        pub exec_context: Option<ExecContext>,
        pub fee: u64,
        pub id: Vec<u8>,
        pub instructions: Vec<SerializableInstruction>,
        pub name: String,
        pub next_instruction: Option<SerializableInstruction>,
        pub paused: bool,
        pub rate_limit: u64,
        pub trigger: Trigger, 
}