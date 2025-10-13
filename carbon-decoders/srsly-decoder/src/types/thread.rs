
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
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
